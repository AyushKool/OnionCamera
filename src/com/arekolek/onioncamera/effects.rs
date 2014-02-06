
#pragma version(1)
#pragma rs java_package_name(com.arekolek.onioncamera)
#pragma rs_fp_relaxed

static rs_allocation in;
static uint32_t maxX, maxY;
static const uint32_t zero = 0;

void set_input(rs_allocation a) {
	in = a;
	maxX = rsAllocationGetDimX(a) - 1;
	maxY = rsAllocationGetDimY(a) - 1;
}

// TODO use rs_sampler instead? (supposedly it's worse)
// TODO bind allocation to array instead?
// TODO add rs_fp_relaxed
// TODO rsForeach in same file makes it nonthreadable?
// TODO run only on a subset of the input (launch options)
// TODO no race conditions in histogram?

inline static uchar4 getElementAt_uchar4_clamped(rs_allocation a, uint32_t x, uint32_t y) {
	return rsGetElementAt_uchar4(a, clamp(x, zero, maxX), clamp(y, zero, maxY));
}

inline static float4 getElementAt_unpack(rs_allocation a, uint32_t x, uint32_t y) {
	return rsUnpackColor8888(getElementAt_uchar4_clamped(a, x, y));
}

uchar4 __attribute__((kernel)) blur(uint32_t x, uint32_t y) {
    float3 pixel = getElementAt_unpack(in, x, y).rgb * 0.12f;
    
    int o = 6;
    
    pixel += getElementAt_unpack(in, x, y-o).rgb * 0.12f;
    pixel += getElementAt_unpack(in, x-o, y).rgb * 0.12f;
    pixel += getElementAt_unpack(in, x, y+o).rgb * 0.12f;
    pixel += getElementAt_unpack(in, x+o, y).rgb * 0.12f;
    
    o = 3;
    
    pixel += getElementAt_unpack(in, x-o, y-o).rgb * 0.1f;
    pixel += getElementAt_unpack(in, x+o, y-o).rgb * 0.1f;
    pixel += getElementAt_unpack(in, x+o, y+o).rgb * 0.1f;
    pixel += getElementAt_unpack(in, x-o, y+o).rgb * 0.1f;
    
    return rsPackColorTo8888(pixel);
}

uchar4 __attribute__((kernel)) edges(uint32_t x, uint32_t y) {
    float4 pixel = getElementAt_unpack(in, x, y) * -4;
    
    int o = 1;
    
    pixel += getElementAt_unpack(in, x, y-o);
    pixel += getElementAt_unpack(in, x-o, y);
    pixel += getElementAt_unpack(in, x, y+o);
    pixel += getElementAt_unpack(in, x+o, y);
    
    pixel.a = 1;
    
    return rsPackColorTo8888(pixel);
}
