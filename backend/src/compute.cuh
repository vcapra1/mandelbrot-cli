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
    uint32_t iterations;
	uint32_t num;
	Pixel pixels[];
} RenderData;

extern "C" {
	uint32_t cuda_compute(RenderData data, uint32_t iterations);
}
