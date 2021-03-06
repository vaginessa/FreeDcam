#pragma version(1)
    #pragma rs java_package_name(freed.utils)
    #pragma rs_fp_relaxed

rs_allocation gCurrentFrame;
rs_allocation gLastFrame;
int Width;
int Height;
bool yuvinput;

//BRIGHTNESS
float brightness;

uchar4 __attribute__((kernel)) processBrightness(uint32_t x, uint32_t y) {
    float4 f4 = rsUnpackColor8888(rsGetElementAt_uchar4(gCurrentFrame, x,y));
        f4.r += brightness;

        if(f4.r > 1){
        	f4.r = 1;
        }

        f4.g += brightness;
        if(f4.g > 1){
        	f4.g = 1;
        }

        f4.b += brightness;
        if(f4.b > 1){
        	f4.b = 1;
        }

     return rsPackColorTo8888(f4);
}


//CONTRAST
float contrastM = 0.f;
float contrastC = 0.f;

void setContrast(float v) {
    contrastM = pow(2.f, v / 100.f);
    contrastC = 127.f - contrastM * 127.f;
}

uchar4 __attribute__((kernel)) processContrast(uint32_t x, uint32_t y)
{
            float3 v = convert_float3(rsGetElementAt_uchar4(gCurrentFrame, x, y).rgb) * contrastM + contrastC;
            uchar4 o;
            o.rgb = convert_uchar3(clamp(v, 0.f, 255.f));
            o.a = 0xff;
            return o;
}

//FOCUSPEAK

uchar4 __attribute__((kernel)) focuspeakcam2(uint32_t x, uint32_t y) {
    uchar4 curPixel;
    curPixel.r = rsGetElementAtYuv_uchar_Y(gCurrentFrame, x, y);
    curPixel.g = rsGetElementAtYuv_uchar_U(gCurrentFrame, x, y);
    curPixel.b = rsGetElementAtYuv_uchar_V(gCurrentFrame, x, y);
    int dx = x + ((x == 0) ? 1 : -1);
    int sum = 0;
    int tmp;
    tmp = rsGetElementAtYuv_uchar_Y(gCurrentFrame, dx, y) - curPixel.r;
    sum += tmp * tmp;
    tmp = rsGetElementAtYuv_uchar_U(gCurrentFrame, dx, y) - curPixel.g;
    sum += tmp * tmp;
    tmp = rsGetElementAtYuv_uchar_V(gCurrentFrame, dx, y) - curPixel.b;
    sum += tmp * tmp;
    int dy = y + ((y == 0) ? 1 : -1);
    tmp = rsGetElementAtYuv_uchar_Y(gCurrentFrame, x, dy) - curPixel.r;
    sum += tmp * tmp;
    tmp = rsGetElementAtYuv_uchar_U(gCurrentFrame, x, dy) - curPixel.g;
    sum += tmp * tmp;
    tmp = rsGetElementAtYuv_uchar_V(gCurrentFrame, x, dy) - curPixel.b;
    sum += tmp * tmp;
    sum >>= 9;
    sum *= sum * sum;
    curPixel.a = 255;
    uchar4 mergedPixel = curPixel;
    int4 rgb;
    rgb.r = mergedPixel.r +
            mergedPixel.b * 1436 / 1024 - 179 + sum;
    rgb.g = mergedPixel.r -
            mergedPixel.g * 46549 / 131072 + 44 -
            mergedPixel.b * 93604 / 131072 + 91 + sum;
    rgb.b = mergedPixel.r +
            mergedPixel.g * 1814 / 1024 - 227;
    rgb.a = 255;
    // Write out merged HDR result
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);
    uchar4 out = convert_uchar4(rgb);
    return out;
}

uchar4 __attribute__((kernel)) focuspeakcam1(uint32_t x, uint32_t y) {
    uchar4 curPixel;
    curPixel.r = rsGetElementAtYuv_uchar_Y(gCurrentFrame, x, y);
    curPixel.g = rsGetElementAtYuv_uchar_U(gCurrentFrame, x, y);
    curPixel.b = rsGetElementAtYuv_uchar_V(gCurrentFrame, x, y);
    int dx = x + ((x == 0) ? 1 : -1);
    int sum = 0;
    int tmp;
    tmp = rsGetElementAtYuv_uchar_Y(gCurrentFrame, dx, y) - curPixel.r;
    sum += tmp * tmp;
    tmp = rsGetElementAtYuv_uchar_U(gCurrentFrame, dx, y) - curPixel.g;
    sum += tmp * tmp;
    tmp = rsGetElementAtYuv_uchar_V(gCurrentFrame, dx, y) - curPixel.b;
    sum += tmp * tmp;
    int dy = y + ((y == 0) ? 1 : -1);
    tmp = rsGetElementAtYuv_uchar_Y(gCurrentFrame, x, dy) - curPixel.r;
    sum += tmp * tmp;
    tmp = rsGetElementAtYuv_uchar_U(gCurrentFrame, x, dy) - curPixel.g;
    sum += tmp * tmp;
    tmp = rsGetElementAtYuv_uchar_V(gCurrentFrame, x, dy) - curPixel.b;
    sum += tmp * tmp;
    sum >>= 9;
    sum *= sum * sum;
    curPixel.a = 255;
    uchar4 mergedPixel = curPixel;
    int4 rgb;
    rgb.r = mergedPixel.r +
            mergedPixel.b * 1436 / 1024 - 179 + sum;
    rgb.g = mergedPixel.r -
            mergedPixel.g * 46549 / 131072 + 44 -
            mergedPixel.b * 93604 / 131072 + 91 + sum;
    rgb.b = mergedPixel.r +
            mergedPixel.g * 1814 / 1024 - 227;
    rgb.a = 255;
    // Write out merged HDR result
    int factor = 2;
    if(rgb.r >=255 -factor && rgb.g >= 255 -factor && rgb.b >= 255 -factor)
    {
        rgb.r =255; rgb.g = 0;  rgb.b = 0;
    }
    if(rgb.r <=0 +factor && rgb.g <= 0 +factor && rgb.b <= 0 +factor)
    {
        rgb.r =0; rgb.g = 0;  rgb.b = 255;
    }
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);

    uchar4 out = convert_uchar4(rgb);

    return out;
}

uchar4 __attribute__((kernel)) focuspeaksony(uint32_t x, uint32_t y) {
    uchar4 curPixel;
    curPixel = rsGetElementAt_uchar4(gCurrentFrame, x, y);
    //rsDebug("CurPixel", curPixel);

    int dx = x + ((x == 0) ? 1 : -1);
    //rsDebug("dx", dx);
    int sum = 0;
    uchar4 tmpPix = rsGetElementAt_uchar4(gCurrentFrame, dx, y);
    int tmp;
    tmp = tmpPix.r - curPixel.r;
    sum += tmp * tmp;
    tmp = tmpPix.g - curPixel.g;
    sum += tmp * tmp;
    tmp = tmpPix.b - curPixel.b;
    sum += tmp * tmp;

    int dy = y + ((y == 0) ? 1 : -1);
    tmpPix = rsGetElementAt_uchar4(gCurrentFrame, x, dy);
    tmp = tmpPix.r - curPixel.r;
    sum += tmp * tmp;
    tmp = tmpPix.g - curPixel.g;
    sum += tmp * tmp;
    tmp = tmpPix.b - curPixel.b;
    sum += tmp * tmp;

    sum >>= 9;
    sum *= sum * sum;
    int4 rgb;
    uchar4 mergedPixel = curPixel;
    //rsDebug("curPixel", curPixel);
    rgb.r = mergedPixel.r  + sum;
    rgb.g = mergedPixel.g + sum;
    rgb.b = mergedPixel.b + sum;
    rgb.a = 255;
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);

    //rsDebug("rgb", rgb);
    uchar4 out = convert_uchar4(rgb);
    return out;
}


//IMAGE STACK

typedef struct MinMaxPixel
{
    uchar4 min;
    uchar4 max;
} MinMaxPixel_t;

MinMaxPixel_t *medianMinMaxPixel;

float4 __attribute__((kernel))getRgb(uint32_t x, uint32_t y)
{
    uchar4 curPixel;
    if(yuvinput)
    {
        curPixel.r = rsGetElementAtYuv_uchar_Y(gCurrentFrame, x, y);
        curPixel.g = rsGetElementAtYuv_uchar_U(gCurrentFrame, x, y);
        curPixel.b = rsGetElementAtYuv_uchar_V(gCurrentFrame, x, y);
        return rsUnpackColor8888(rsYuvToRGBA_uchar4(curPixel.r,curPixel.g,curPixel.b));
    }
    else
    {
        return rsUnpackColor8888(rsGetElementAt_uchar4(gCurrentFrame, x, y));
    }
}

uchar4 __attribute__((kernel))getucharRgb(uint32_t x, uint32_t y)
{
    uchar4 curPixel;
    if(yuvinput)
    {
        curPixel.r = rsGetElementAtYuv_uchar_Y(gCurrentFrame, x, y);
        curPixel.g = rsGetElementAtYuv_uchar_U(gCurrentFrame, x, y);
        curPixel.b = rsGetElementAtYuv_uchar_V(gCurrentFrame, x, y);
        return rsYuvToRGBA_uchar4(curPixel.r,curPixel.g,curPixel.b);
    }
    else
    {
        return rsGetElementAt_uchar4(gCurrentFrame, x, y);
    }
}

uchar4 __attribute__((kernel))getRgb_uchar4(uint32_t x, uint32_t y)
{
    uchar4 curPixel;
    if(yuvinput)
    {
        curPixel.r = rsGetElementAtYuv_uchar_Y(gCurrentFrame, x, y);
        curPixel.g = rsGetElementAtYuv_uchar_U(gCurrentFrame, x, y);
        curPixel.b = rsGetElementAtYuv_uchar_V(gCurrentFrame, x, y);
        curPixel.a = 255;
        return rsYuvToRGBA_uchar4(curPixel.r,curPixel.g,curPixel.b);
    }
    else
    {
        return rsGetElementAt_uchar4(gCurrentFrame, x, y);
    }
}

uchar4 __attribute__((kernel)) stackimage_avarage(uint32_t x, uint32_t y) {
    float4 curPixel, lastPixel, merged;
    uchar4 rgb;
    curPixel = getRgb(x,y);
    lastPixel = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y));
    merged = (curPixel + lastPixel)/2;
    rgb = rsPackColorTo8888(merged);
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);
    return rgb;
}

uchar4 __attribute__((kernel)) stackimage_exposure(uint32_t x, uint32_t y) {
    float4 curPixel, lastPixel, merged;
    uchar4 rgb;
    curPixel = getRgb(x,y);
    lastPixel = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y));
    merged = (curPixel*2 + lastPixel*4)/4;
    rgb = rsPackColorTo8888(merged);
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);
    return rgb;
}

// takes two pixel sample shows a 3x3 array PIX are the used one
// PIX PIX pix
// pix pix pix
// pix pix pix
uchar4 __attribute__((kernel)) stackimage_avarage1x2(uint32_t x, uint32_t y)
{
    float4 curPixel, lastPixel, curPixel1,lastPixel1,merged;
    uchar4 rgb;
    if(x < Width)
    {
        curPixel = getRgb(x,y);
        curPixel1 = getRgb(x+1, y);
        lastPixel = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y));
        lastPixel1 = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x+1, y));

        merged = (curPixel + lastPixel + curPixel1 + lastPixel1)/4;
        rgb = rsPackColorTo8888(merged);
    }
    else
    {
        curPixel = getRgb(x,y);
        lastPixel = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y));
        rgb = rsPackColorTo8888(merged);
    }
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);
    return rgb;
}

// takes three pixel sample shows a 3x3 array PIX are the used one
//  PIX   PIX  pix
//  PIX   pix  pix
//  pix   pix  pix
uchar4 __attribute__((kernel)) stackimage_avarage1x3(uint32_t x, uint32_t y)
{
    float4 cPix, lPix, cPixX1,lPixX1, cPixY1, lPixY1 , mergedPix;

    uchar4 rgb;
    //rsDebug("Width", x);
    if(x+1 < Width && y+1 < Height)
    {
        cPix = getRgb(x,y);
        cPixX1 = getRgb(x+1,y);
        cPixY1 = getRgb(x, y+1);
        lPix = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y));
        lPixX1 = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x+1, y));
        lPixY1 = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y+1));
        mergedPix = (cPix + lPix + cPixX1 + lPixX1 + cPixY1 + lPixY1)/6;
        rgb = rsPackColorTo8888(mergedPix);
    }
    else
    {
        cPix = getRgb(x, y);
        lPix = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y));
        mergedPix = (cPix + lPix)/2;
        rgb = rsPackColorTo8888(mergedPix);

    }
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);
    return rgb;
}

// takes three pixel sample shows a 3x3 array PIX are the used one
//  PIX1   PIX2  PIX3
//  PIX4   PIX5  PIX6
//  PIX7   PIX8  PIX9
uchar4 __attribute__((kernel)) stackimage_avarage3x3(uint32_t x, uint32_t y)
{
    float4 PIX1, PIX2, PIX3,PIX4, PIX5, PIX6 , PIX7, PIX8, PIX9,
    PIX1L, PIX2L, PIX3L,PIX4L, PIX5L, PIX6L , PIX7L, PIX8L, PIX9L, mergedPix;

    uchar4 rgb;
    //rsDebug("Width", x);
    if(x > 0 && x+1 < Width && y > 0 && y+1 < Height)
    {
        PIX5 = getRgb(x,y); // pixel that get merged
        PIX1 = getRgb(x-1, y-1);
        PIX2 = getRgb(x, y-1);
        PIX3 = getRgb(x+1, y-1);
        PIX4 = getRgb(x-1, y);
        PIX6 = getRgb(x+1, y);
        PIX7 = getRgb(x+1, y+1);
        PIX8 = getRgb(x, y+1);
        PIX9 = getRgb(x+1, y+1);

        PIX5L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y)); // pixel that get merged
        PIX1L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x-1, y-1));
        PIX2L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y-1));
        PIX3L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x+1, y-1));
        PIX4L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x-1, y));
        PIX6L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x+1, y));
        PIX7L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x+1, y+1));
        PIX8L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y+1));
        PIX9L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x+1, y+1));

        mergedPix = (PIX1 + PIX2 +PIX3+PIX4+PIX5+PIX6+PIX7+PIX8+PIX9+PIX1L + PIX2L +PIX3L+PIX4L+PIX5L+PIX6L+PIX7L+PIX8L+PIX9L)/18;
        rgb = rsPackColorTo8888(mergedPix);
    }
    else
    {
        PIX5 = getRgb(x, y);
        PIX5L = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y));
        mergedPix = (PIX5 + PIX5L)/2;
        rgb = rsPackColorTo8888(mergedPix);

    }
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);
    return rgb;
}

uchar4 __attribute__((kernel)) stackimage_lighten(uint32_t x, uint32_t y)
{
    float4 curPixel, lastPixel;
    uchar4 rgb;
    curPixel = getRgb(x, y);
    lastPixel = rsUnpackColor8888(rsGetElementAt_uchar4(gLastFrame, x, y));
    if(curPixel.x > lastPixel.x && curPixel.y > lastPixel.y && curPixel.z > lastPixel.z)
        rgb = rsPackColorTo8888(curPixel);
    else
        rgb = rsPackColorTo8888(lastPixel);
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);
    return rgb;
}

uchar4 __attribute__((kernel)) stackimage_lightenV(uint32_t x, uint32_t y)
{
    uchar4 curPixel, lastPixel;
    uchar4 rgb;
    curPixel = getucharRgb(x, y);
    int V1 = ((curPixel.r<<1)+(curPixel.g<<2)+curPixel.b)>>3;
    lastPixel = rsGetElementAt_uchar4(gLastFrame, x, y);
    int V2 =((lastPixel.r<<1)+(lastPixel.g<<2)+lastPixel.b)>>3;
    if(V1 > V2)
        rgb = curPixel;
    else
        rgb = lastPixel;
   rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
   rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
   rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);
    return rgb;
}

uchar4 __attribute__((kernel)) stackimage_median(uint32_t x, uint32_t y)
{
    uchar4 curPixel, lastPixel;
    curPixel = getRgb_uchar4(x, y);
    struct MinMaxPixel t = medianMinMaxPixel[x+y];
    if(curPixel.r < t.min.r && curPixel.g < t.min.g && curPixel.b < t.min.b)
        t.min = curPixel;
    else if(curPixel.r > t.max.r && curPixel.g > t.max.g && curPixel.b > t.max.b)
        t.max = curPixel;
    uchar4 rgb = t.max - t.min;
    rgb.r = ( rgb.r > 255 )? 255 : (( rgb.r < 0 )? 0 : rgb.r);
    rgb.g = ( rgb.g > 255 )? 255 : (( rgb.g < 0 )? 0 : rgb.g);
    rgb.b = ( rgb.b > 255 )? 255 : (( rgb.b < 0 )? 0 : rgb.b);
    return rgb;
}


///INTERPOLATE IMAGE2x

uchar4 __attribute__((kernel)) fillPixelInterpolate(uint32_t x, uint32_t y)
{
    uchar4 inPixel1,inPixel2, outpix;
    int outpixPosX= x *2;
    int outpixPosY= y*2;
    inPixel1 = rsGetElementAt_uchar4(gCurrentFrame,x, y);

    rsSetElementAt_uchar4(gLastFrame, inPixel1, outpixPosX, outpixPosY);
    if(x+1 < Width){
        rsSetElementAt_uchar4(gLastFrame, inPixel1, outpixPosX+1, outpixPosY);
    }
    if(y+1 < Height)
        rsSetElementAt_uchar4(gLastFrame, inPixel1, outpixPosX, outpixPosY+1);
    if(y+1 < Height && x+1 < Width)
        rsSetElementAt_uchar4(gLastFrame, inPixel1, outpixPosX+1, outpixPosY+1);
    return inPixel1;
}

uchar4 __attribute__((kernel)) clear(uint32_t x, uint32_t y)
{
    uchar4 inPixel1,inPixel2, outpix;
    inPixel1 = rsGetElementAt_uchar4(gLastFrame,x, y);
    inPixel1.r = 0;
    inPixel1.g = 0;
    inPixel1.b = 0;
    return inPixel1;
}


//GRAYSCALE
uchar4 __attribute__((kernel)) grayscale(uint32_t x, uint32_t y) {
    uchar4 pix = rsGetElementAt_uchar4(gCurrentFrame, x,y);
    if(pix.r < 10 && pix.g < 10 && pix.b < 10)
    {
        pix.r = 10;
        pix.g = 10;
        pix.b = 10;
    }
    if(pix.r < 20 && pix.g < 20 && pix.b < 20)
    {
        pix.r = 20;
        pix.g = 20;
        pix.b = 20;
    }
    else if(pix.r <30 && pix.g < 30 && pix.b < 30)
    {
        pix.r = 30;
        pix.g = 30;
        pix.b = 30;
    }
    else if(pix.r < 40 && pix.g < 40 && pix.b < 40)
    {
        pix.r = 40;
        pix.g = 40;
        pix.b = 40;
    }
    else if(pix.r < 50 && pix.g < 50 && pix.b < 50)
    {
        pix.r = 50;
        pix.g = 50;
        pix.b = 50;
    }
    else if(pix.r < 60 && pix.g < 60 && pix.b < 60)
    {
        pix.r = 60;
        pix.g = 60;
        pix.b = 60;
    }
    else if(pix.r < 70 && pix.g < 70 && pix.b < 70)
    {
        pix.r = 70;
        pix.g = 70;
        pix.b = 70;
    }
    else if(pix.r < 80 && pix.g < 80 && pix.b < 80)
    {
        pix.r = 80;
        pix.g = 80;
        pix.b = 80;
    }
    else if(pix.r < 90 && pix.g < 90 && pix.b < 90)
    {
        pix.r = 90;
        pix.g = 90;
        pix.b = 90;
    }
    else if(pix.r < 100 && pix.g < 100 && pix.b < 100)
    {
        pix.r = 100;
        pix.g = 100;
        pix.b = 100;
    }
    else if(pix.r < 110 && pix.g < 110 && pix.b < 110)
    {
        pix.r = 110;
        pix.g = 110;
        pix.b = 110;
    }
    else if(pix.r < 120 && pix.g < 120 && pix.b < 120)
    {
        pix.r = 120;
        pix.g = 120;
        pix.b = 120;
    }
    else if(pix.r < 130 && pix.g < 130 && pix.b < 130)
    {
        pix.r = 130;
        pix.g = 130;
        pix.b = 130;
    }
    else if(pix.r < 140 && pix.g < 140 && pix.b < 140)
    {
        pix.r = 140;
        pix.g = 140;
        pix.b = 140;
    }
    else if(pix.r < 150 && pix.g < 150 && pix.b < 150)
    {
        pix.r = 150;
        pix.g = 150;
        pix.b = 150;
    }
    else if(pix.r < 160 && pix.g < 160 && pix.b < 160)
    {
        pix.r = 160;
        pix.g = 160;
        pix.b = 160;
    }
    else if(pix.r < 170 && pix.g < 170 && pix.b < 170)
    {
        pix.r = 170;
        pix.g = 170;
        pix.b = 170;
    }
    else if(pix.r < 180 && pix.g < 180 && pix.b < 180)
    {
        pix.r = 180;
        pix.g = 180;
        pix.b = 180;
    }
    else if(pix.r < 190 && pix.g < 190 && pix.b < 190)
    {
        pix.r = 190;
        pix.g = 190;
        pix.b = 190;
    }
    else if(pix.r < 200 && pix.g < 200 && pix.b < 200)
    {
        pix.r = 200;
        pix.g = 200;
        pix.b = 200;
    }
    else if(pix.r < 210 && pix.g < 210 && pix.b < 210)
    {
        pix.r = 210;
        pix.g = 210;
        pix.b = 210;
    }
    else if(pix.r < 220 && pix.g < 220 && pix.b < 220)
    {
        pix.r = 220;
        pix.g = 220;
        pix.b = 220;
    }
    else if(pix.r < 230 && pix.g < 230 && pix.b < 230)
    {
        pix.r = 230;
        pix.g = 230;
        pix.b = 230;
    }
    else if(pix.r < 240 && pix.g < 240 && pix.b < 240)
    {
        pix.r = 240;
        pix.g = 240;
        pix.b = 240;
    }
    else
    {
        pix.r = 255;
        pix.g = 255;
        pix.b = 255;
    }

     return pix;
}