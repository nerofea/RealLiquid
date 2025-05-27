// Foreign Function Interface (FFI) 
#include "signer.hpp"
#include "network_parameters.hpp"
#include <nlohmann/json.hpp>
#include <string>

extern "C" {
    static green::signer* GLOBAL_SIGNER = nullptr;

    void gdk_init_signer(const char* json_creds) {
        nlohmann::json creds = nlohmann::json::parse(json_creds);
        nlohmann::json device = nlohmann::json::object();
        nlohmann::json net_params_json = R"({"main_net": true, "liquid": false, "btc_version": 0})"_json;
        green::network_parameters net_params(net_params_json);
        GLOBAL_SIGNER = new green::signer(net_params, device, creds);
    }

    const char* gdk_get_xpub() {
        static std::steing result;
        result = GLOBAL_SIGNER->get_master_bip32_xpub();
        return result.c_str();
    }

    void gdk_free_signer() {
        delete GLOBAL_SIGNER;
        GLOBAL_SIGNER = nullptr;
    }

    // FFI-safe transaction constructor
    void* Tx_from_hex(const char* tx_hex, bool is_liquid) {
        auto* tx = new green::Tx(std::string(tx_hex), is_liquid);
        return static_cast<void*>(tx);
    }

    // FFI-safe get_signature_hash
    void Tx_get_signature_hash(void* tx_ptr, void* session_ptr, const char* json_inputs, size_t index, uint32_t sighash_flags, uint8_t* out_hash32) {
        auto* tx = static_cast<green::Tx*>(tx_ptr);
        auto* session = static_cast<green::session_impl*>(session_ptr);
        auto inputs = nlohmann::json::parse(json_inputs);
        auto hash = tx->get_signature_hash(*session, inputs, index, sighash_flags);
        std::memcpy(out_hash32, hash.data(), 32);
    }

    //FFI-safe sign_transaction (returns JSON array of hex signatures)
    char* sign_transaction_json(void* session_ptr, void* tx_ptr, const char* json_inputs) {
        auto* tx = static_cast<green::Tx*>(tx_ptr);
        auto* session = static_cast<green::session_impl*>(session_ptr);
        auto inputs = nlohmann::json::parse(json_inputs);
        auto sigs = green::sign_transaction(*session, *tx, inputs);
        std::string result = nlohmann::json(sigs).dump();
        char* out = new char[result.size() + 1];
        std::memcpy(out, result.c_str(), result.size() + 1);
        return out;
    }

    void free_tx(void* tx_ptr) {
        delete static_cast<green::Tx*>(tx_ptr);
    }

    void free_str(char* ptr) {
        delete[] ptr;
    }
}" 