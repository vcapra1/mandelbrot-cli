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

__global__ void compute(Pixel *pixels, unsigned long width, unsigned long height, unsigned long iterations) {
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
		}
	}
}

extern "C" {
	uint32_t cuda_compute(uint32_t iterations, RenderData data, uint32_t *progress) {
		// Print information to ensure it was all transferred properly
		//printf("Iterations: %u\n", iterations);
		//printf("RenderData:\n");
		//printf("  Iterations: %u\n", data.iterations); 
		//printf("  Num: %u\n", data.num);
		//printf("  Pixels: %p\n", (void *)data.pixels);
		//for (unsigned int i = 0; i < data.num; i++) {
		//	if (i == (unsigned int) 7 || i == (unsigned int) (data.num / 2 + 100)) {
		//		printf("    ...\n");
		//	}
		//	if (i >= (unsigned int) 7 && i < (unsigned int) (data.num / 2 + 97)) {
		//		continue;
		//	}
		//	if (i >= (unsigned int) (data.num / 2 + 100) && i < (unsigned int) (data.num - 3)) {
		//		continue;
		//	}

		//	printf("    %d: (i: %d, d: %s, c: (%f, %f), z: (%f, %f))\n", i, data.pixels[i].i,
		//		data.pixels[i].d ? "true" : "false", data.pixels[i].c.real, data.pixels[i].c.imag,
		//		data.pixels[i].z.real, data.pixels[i].z.imag);
		//}

		if (data.width > 2097120 || data.height > 2097120) {
			// Too big :( TODO: not really, we can go quite a bit bigger, but we'll do that later
			return 99999;
		}

		// Keep track of errors
		cudaError_t status = cudaSuccess;

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
		compute<<<gridSize, blockSize>>>(pixels, data.width, data.height, iterations);
		status = cudaPeekAtLastError();
		
		if (status != cudaSuccess) { return status; }

		status = cudaDeviceSynchronize();

		if (status != cudaSuccess) { return status; }

		// Copy pixels data back to original memory
		memcpy(data.pixels, pixels, data_length);

		// Free memory
		cudaFree(pixels);

		// Set progress to 100%
		*progress = 100;

		return 0;
	}
}
