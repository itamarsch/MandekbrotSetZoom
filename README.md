# Mandelbrot Set Zoom

This is just another Mandelbrot Set zoom project.
There are two implementations here, one in the cpu directory and one in the gpu directory

# Installation
Follow instuctions for setting up SDL and OCL on your machine

# Run 
To run CPU version: `RUSTFLAGS="-C target-cpu=native" cargo run --release -F "cpu"`  
To run GPU version: `RUSTFLAGS="-C target-cpu=native" cargo run --release -F "gpu"`


# Preview
![](/mandelbrot.png )

![](/mandelbrot2.png)