#include <inttypes.h>

typedef double Real;

typedef struct {
	Real real;
	Real imag;
} Complex;

typedef struct {
	Complex c;
	Complex z;
	uint32_t i;
	bool d;
} Pixel;

typedef struct {
	Pixel *pixels;
    uint32_t iterations;
	uint32_t num;
	uint32_t width;
	uint32_t height;
} RenderData;

extern "C" {
	uint32_t cuda_compute(uint32_t iterations, RenderData data, void **progress);
}
