#include "compute.cuh"

#include <stdio.h>
#include <string.h>
#include <unistd.h>

__device__ Real mul(Real a, Real b) { return a * b; }
__device__ Real add(Real a, Real b) { return a + b; }
__device__ Real sub(Real a, Real b) { return a - b; }
__device__ Real mag_sq(Complex c) { return add(mul(c.real, c.real), mul(c.imag, c.imag)); }

__device__ Complex f(Complex z, Complex c) {
	Real real = add(sub(mul(z.real, z.real), mul(z.imag, z.imag)), c.real);
	Real w = mul(z.real, z.imag);
	Real imag = add(add(w, w), c.imag);

	return { real, imag };
}

__global__ void compute(Pixel *pixels, 
						unsigned long width, 
						unsigned long height, 
						unsigned long iterations, 
						unsigned long long *progress) {

	// Figure out which pixel this thread is responsible for
	unsigned long x = blockIdx.x * blockDim.x + threadIdx.x;
	if (x < width) {
		unsigned long y = blockIdx.y * blockDim.y + threadIdx.y;
		if (y < height) {
			unsigned long idx = x + y * width;

			// Get a pointer to our pixel
			Pixel *pixel = &pixels[idx];

			// Loop until the pixel diverges, or the max iterations is reached
			while (pixel->i < iterations && !pixel->d) {
				pixel->z = f(pixel->z, pixel->c);
				pixel->i += 1;

				// Check to see if it's diverged
				if (mag_sq(pixel->z) > 4.0) {
					pixel->d = true;
				}
			}

			// Increment the progress
			atomicAdd(progress, 1);
		}
	}
}

extern "C" {
	uint32_t cuda_compute(uint32_t iterations, RenderData data, void **progress) {
		// Make sure the image isn't too big
		if (data.width > 2097120 || data.height > 2097120) {
			// Too big :( TODO: not really, we can go quite a bit bigger, but we'll do that later
			return 99999;
		}

		// Keep track of errors
		cudaError_t status = cudaSuccess;

		// Allocate managed memory for the progress
		unsigned long long *progress_shared;
		status = cudaMallocManaged((void **)&progress_shared, sizeof(unsigned long long));

		if (status != cudaSuccess) { return status; }

		// Set the progress to zero
		*progress_shared = 0;

		// Pass the reference to the progress back through the double pointer
		*progress = (void *)progress_shared;

		// Allocate managed memory for the pixels
		Pixel *pixels;
		unsigned int data_length = sizeof(Pixel) * data.num;
		status = cudaMallocManaged((void **)&pixels, data_length);

		if (status != cudaSuccess) { return status; }

		// Copy pixels data from host to managed memory
		memcpy(pixels, data.pixels, data_length);

		// Calculate grid and block sizes
		dim3 blockSize(32, 32, 1);
		dim3 gridSize(data.width / blockSize.x, data.height / blockSize.y, 1);

		// Round up
		if (data.width % blockSize.x) { gridSize.x += 1; }
		if (data.height % blockSize.y) { gridSize.y += 1; }

		// Run kernel
		compute<<<gridSize, blockSize>>>(pixels, data.width, data.height, iterations, progress_shared);
		status = cudaPeekAtLastError();

		if (status != cudaSuccess) { return status; }

		status = cudaDeviceSynchronize();

		if (status != cudaSuccess) { return status; }

		// Copy pixels data back to original memory
		memcpy(data.pixels, pixels, data_length);

		// Clear the progress reference
		*progress = 0;
		
		// Wait to make sure the progress thread (in Rust) doesn't try to access freed memory
		sleep(1);

		// Free memory
		cudaFree(progress_shared);
		progress_shared = 0;
		
		cudaFree(pixels);
		pixels = 0;

		return 0;
	}
}
