use cust::prelude::*;
use std::error::Error;

trait ToVecF32 {
    fn to_vec_f32(&self) -> Vec<f32>;
}

impl ToVec32 for MessageTile {
    fn to_vec_f32(&self) -> Vec<f32> { 
        // flatten the data into f32 numerical vectors, preparation for GPU inputs
        let Q: Vec<f32> = flatten_tile_data(&Q_tiles);  //Message tile
        let K: Vec<f32> = flatten_tile_data(&K_tiles);  // Policy tile
        let V: Vec<f32> = flatten_tile_data(&V_tiles); // Signer tile
    }
}

pub fn launch_flash_kernel<T: ToVecF32>(
    Q_tiles: &[Vec<T>],
    K_tiles: &[Vec<T>],
    V_tiles: &[Vec<T>],
    total_output_len: usize,
) -> Result<(), Box<dyn Error>> {
    // Load the PTX file
    let ptx_code = std::fs::read_to_string("./kernels/flash_sign.ptx")?;

    // Initialize CUDA context
    let _ctx = cust::quick_init()?;
    let module = Module::from_ptx(ptx_code, &[])?;
    let stream = Stream::new(StreamFlags::DEFAULT, None)?;
    // Load the flash attention sign function from PTX
    let function = module.get_function("flash_sign_kernel")?;

    // Flatten the MessageTile, PolicyTile, SignerTile into Vec<f32> numerical vectors
    fn flatten_tile_data<T: ToVecF32>(tiles: &[Vec<T>]) -> Vec<f32> {
        tiles.iter()
        .flat_map(|tile| tile.iter().flat_map(|t| t.to_vec_f32()))
        .collect()
    }
    
    // flatten the data into f32 numerical vectors, preparation for GPU inputs
    let Q: Vec<f32> = flatten_tile_data(&Q_tiles);  //Message tile
    let K: Vec<f32> = flatten_tile_data(&K_tiles);  // Policy tile
    let V: Vec<f32> = flatten_tile_data(&V_tiles); // Signer tile

    let d_Q = DeviceBuffer::from_slice(&(Q))?;
    let d_K = DeviceBuffer::from_slice(&(K))?;
    let d_V = DeviceBuffer::from_slice(&(V))?;
    let mut d_O = DeviceBuffer::<f32>::zeroed(total_output_len)?;

    let tile_size_r: u32 = 64;
    let tile_size_c: u32 = 64;
    
    // Launch the kernel 
    let blocks = 1;
    let threads = 64;
    let shared_mem = 0;

    unsafe {
        cust::launch!(
            function<<blocks, threads, shared_mem, stream>>>(
                d_Q.as_device_ptr(),
                d_K.as_device_ptr(),
                d_V.as_device_ptr(),
                d_O.as_device_ptr(),
                tile_size_r,
                tile_size_c
            )
        )?;
    }

    stream.synchronize()?;
    Ok(())
}