let ptx_code = std::fs::read_to_string("flas_sign.ptx")?;
let context = cust::quick_init()?;
let module = Module::from_ptx(ptx_code, &[])?;
let function = module.get_function("flash_sign_kernel")?;

cust::launch!(
    function<<<blocks, threads, shared_mem, stream>>>(
        d_Q, d_K, d_V, d_O, tile_size_r, tile_size_c
    )
)?;