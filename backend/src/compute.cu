#include "compute.cuh"

#include <stdio.h>

__global__ void compute() {
	
}

extern "C" {
	uint32_t cuda_compute(uint32_t iterations, RenderData data) {
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

		return 0;
	}
}
