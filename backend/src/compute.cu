#include "compute.cuh"

extern "C" {
	uint32_t cuda_compute(RenderData data, uint32_t iterations) {
		return 42;
	}
}
