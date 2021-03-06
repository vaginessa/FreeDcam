package freed.cam.apis.camera2.modules;

import android.annotation.TargetApi;
import android.graphics.ImageFormat;
import android.hardware.camera2.CameraCaptureSession;
import android.hardware.camera2.CameraCharacteristics;
import android.hardware.camera2.CaptureRequest;
import android.hardware.camera2.CaptureResult;
import android.hardware.camera2.DngCreator;
import android.hardware.camera2.TotalCaptureResult;
import android.hardware.camera2.params.ColorSpaceTransform;
import android.location.Location;
import android.media.Image;
import android.media.ImageReader;
import android.os.Build;
import android.support.annotation.NonNull;
import android.support.v4.provider.DocumentFile;
import android.util.Pair;
import android.util.Rational;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.List;

import freed.ActivityInterface;
import freed.cam.apis.basecamera.modules.WorkFinishEvents;
import freed.dng.CustomMatrix;
import freed.dng.DngProfile;
import freed.utils.Log;
import freed.dng.ToneMapProfile;

/**
 * Created by troop on 12.06.2017.
 */


@TargetApi(Build.VERSION_CODES.LOLLIPOP)

public class ImageHolder
{

    public interface RdyToSaveImg
    {
        void onRdyToSaveImg(ImageHolder holder);
    }

    public interface ImageSaveImp
    {
        void saveRawToDng(File fileName, byte[] bytes, float fnumber, float focal, float exposuretime, int iso, int orientation, String wb, DngProfile dngProfile);
        void saveJpeg(File file, byte[] bytes);
    }

    private final String TAG = ImageHolder.class.getSimpleName();
    private CaptureResult captureResult;
    private List<Image> images;
    private CameraCharacteristics characteristics;
    private CustomMatrix customMatrix;
    private ToneMapProfile toneMapProfile;
    private int orientation = 0;
    private Location location;
    private boolean externalSD =false;

    String filepath;

    private boolean isRawCapture = false;
    private boolean forceRawToDng = false;

    private ActivityInterface activityInterface;
    private ImageSaveImp imageSaver;
    private RdyToSaveImg rdyToSaveImg;

    WorkFinishEvents workerfinish;

    public ImageHolder(CameraCharacteristics characteristicss, boolean isRawCapture, ActivityInterface activitiy, ImageSaveImp imageSaver, WorkFinishEvents finish, RdyToSaveImg rdyToSaveImg)
    {
        images = new ArrayList<>();
        this.characteristics = characteristicss;
        this.isRawCapture = isRawCapture;
        this.activityInterface = activitiy;
        this.imageSaver = imageSaver;
        this.workerfinish = finish;
        this.rdyToSaveImg =rdyToSaveImg;
    }


    public void setCustomMatrix(CustomMatrix custmMat)
    {
        this.customMatrix = custmMat;
    }

    public void setToneMapProfile(ToneMapProfile toneMapProfile)
    {
        this.toneMapProfile = toneMapProfile;
    }

    public void setOrientation(int or)
    {
        this.orientation = or;
    }

    public void setFilePath(String path, boolean extSD)
    {
        this.filepath = path;
        this.externalSD = extSD;
    }

    public void setLocation(Location location)
    {
        this.location = location;
    }

    public void setForceRawToDng(boolean force)
    {
        this.forceRawToDng = force;
    }

    public synchronized void SetCaptureResult(CaptureResult captureResult)
    {
        this.captureResult = captureResult;

        try {
            Log.d(TAG, "ColorMatrix1:" + characteristics.get(CameraCharacteristics.SENSOR_COLOR_TRANSFORM1).toString());
            Log.d(TAG, "ColorMatrix2:" + characteristics.get(CameraCharacteristics.SENSOR_COLOR_TRANSFORM2).toString());
            logNeutralMatrix();
            Log.d(TAG, "Transform1:" + characteristics.get(CameraCharacteristics.SENSOR_COLOR_TRANSFORM1).toString());
            Log.d(TAG, "Transform2:" + characteristics.get(CameraCharacteristics.SENSOR_COLOR_TRANSFORM2).toString());
            Log.d(TAG, "Foward1:" + characteristics.get(CameraCharacteristics.SENSOR_FORWARD_MATRIX1).toString());
            Log.d(TAG, "Foward2:" + characteristics.get(CameraCharacteristics.SENSOR_FORWARD_MATRIX2).toString());
            Log.d(TAG, "Reduction1:" + characteristics.get(CameraCharacteristics.SENSOR_CALIBRATION_TRANSFORM1).toString());
            Log.d(TAG, "Reduction2:" + characteristics.get(CameraCharacteristics.SENSOR_CALIBRATION_TRANSFORM2).toString());
            logColorPattern();
            Log.d(TAG, "Blacklvl:" + characteristics.get(CameraCharacteristics.SENSOR_BLACK_LEVEL_PATTERN).getOffsetForIndex(0,0));
        }
        catch (NullPointerException ex)
        {
            Log.WriteEx(ex);
        }
    }

    public synchronized void AddImage(Image image)
    {
        images.add(image);
        Log.d(TAG, "ImageSize:" + image.getPlanes()[0].getBuffer().remaining());
        logImageFormat(image);
        Log.d(TAG,"WxH:" + image.getWidth() +"x"+image.getHeight());
    }

    public synchronized boolean rdyToGetSaved()
    {
        if (isRawCapture)
            return images.size() == 2 && captureResult != null;
        else
            return images.size() == 1 && captureResult != null;
    }

    public final ImageReader.OnImageAvailableListener mOnRawImageAvailableListener = new ImageReader.OnImageAvailableListener()
    {
        @Override
        public void onImageAvailable(final ImageReader reader)
        {

            Image img = null;
            Log.d(TAG, "OnRawAvailible");
            try {
                img = reader.acquireLatestImage();
                AddImage(img);
            }
            catch (IllegalStateException ex)
            {
                if (img != null)
                    img.close();
            }
            if (rdyToGetSaved())
                rdyToSaveImg.onRdyToSaveImg(ImageHolder.this);
        }
    };

    public final CameraCaptureSession.CaptureCallback imageCaptureMetaCallback = new CameraCaptureSession.CaptureCallback()
    {
        @Override
        public void onCaptureCompleted(CameraCaptureSession session, CaptureRequest request, TotalCaptureResult result) {
            Log.d(TAG, "onCaptureCompleted FrameNum:" +result.getFrameNumber());

            Log.d(TAG, "OnCaptureResultAvailible");
            SetCaptureResult(result);
            if (rdyToGetSaved())
                rdyToSaveImg.onRdyToSaveImg(ImageHolder.this);
        }
    };

    public Runnable getRunner()
    {
        return new Runnable() {
            @Override
            public void run() {
                for(int i=0; i< images.size();i++)
                    saveImage(images.get(i),filepath);
                clear();
            }
        };
    }

    public void CLEAR()
    {
        for (Image img : images)
            img.close();
        clear();
    }

    private void clear()
    {
        images = null;
        customMatrix = null;
        captureResult = null;
        location = null;
        characteristics = null;
        activityInterface = null;
        ImageSaveImp imageSaver = null;
    }


    private void saveImage(Image image,String f) {
        File file = null;
        switch (image.getFormat())
        {
            case ImageFormat.JPEG:
                file = new File(f+".jpg");
                process_jpeg(image, file);
                break;
            case ImageFormat.RAW10:
                file = new File(f+".dng");
                process_rawWithDngConverter(image, DngProfile.Mipi,file);
                break;
            case ImageFormat.RAW12:
                file = new File(f+".dng");
                process_rawWithDngConverter(image,DngProfile.Mipi12,file);
                break;
            case ImageFormat.RAW_SENSOR:
                file = new File(f+".dng");
                if(forceRawToDng)
                    process_rawWithDngConverter(image,DngProfile.Plain,file);
                else
                    process_rawSensor(image,file);
                break;
        }
        image.close();
        workerfinish.internalFireOnWorkDone(file);
    }



    @NonNull
    private void process_jpeg(Image image, File file) {

        Log.d(TAG, "Create JPEG");
        ByteBuffer buffer = image.getPlanes()[0].getBuffer();
        byte[] bytes = new byte[buffer.remaining()];
        buffer.get(bytes);
        imageSaver.saveJpeg(file, bytes);
        buffer.clear();
    }


    @NonNull
    private void process_rawSensor(Image image,File file) {
        Log.d(TAG, "Create DNG");

        DngCreator dngCreator = new DngCreator(characteristics, captureResult);
        //Orientation 90 is not a valid EXIF orientation value, android doc says that is valid!
        //The clockwise rotation angle in degrees, relative to the orientation to the camera, that the JPEG picture needs to be rotated by, to be viewed upright.
        try {
            dngCreator.setOrientation(orientation);
        }
        catch (IllegalArgumentException ex)
        {
            Log.WriteEx(ex);
        }

        if (location != null)
            dngCreator.setLocation(location);
        try
        {
            if (!externalSD)
                dngCreator.writeImage(new FileOutputStream(file), image);
            else
            {
                DocumentFile df = activityInterface.getFreeDcamDocumentFolder();
                DocumentFile wr = df.createFile("image/*", file.getName());
                dngCreator.writeImage(activityInterface.getContext().getContentResolver().openOutputStream(wr.getUri()), image);
            }
            activityInterface.ScanFile(file);
        } catch (IOException ex) {
            Log.WriteEx(ex);
        }
    }

    @NonNull
    private void process_rawWithDngConverter(Image image, int rawFormat,File file) {
        Log.d(TAG, "Create DNG VIA RAw2DNG");
        ByteBuffer buffer = image.getPlanes()[0].getBuffer();
        byte[] bytes = new byte[buffer.remaining()];
        buffer.get(bytes);

        float fnum, focal = 0;
        fnum = captureResult.get(CaptureResult.LENS_APERTURE);
        focal = captureResult.get(CaptureResult.LENS_FOCAL_LENGTH);
        Log.d("Freedcam RawCM2",String.valueOf(bytes.length));

        int mISO = captureResult.get(CaptureResult.SENSOR_SENSITIVITY);
        double mExposuretime = captureResult.get(CaptureResult.SENSOR_EXPOSURE_TIME).doubleValue() / 1000000000;
        final DngProfile prof = getDngProfile(rawFormat, image);
        prof.toneMapProfile = toneMapProfile;
        imageSaver.saveRawToDng(file, bytes, fnum,focal,(float)mExposuretime,mISO, orientation,null,prof);
        bytes = null;
        buffer = null;
    }

    @NonNull
    private DngProfile getDngProfile(int rawFormat, Image image) {
        int black  = characteristics.get(CameraCharacteristics.SENSOR_BLACK_LEVEL_PATTERN).getOffsetForIndex(0,0);
        int c= characteristics.get(CameraCharacteristics.SENSOR_INFO_COLOR_FILTER_ARRANGEMENT);
        String colorpattern;
        int[] cfaOut = new int[4];
        switch (c)
        {
            case 1:
                colorpattern = DngProfile.GRBG;
                cfaOut[0] = 1;
                cfaOut[1] = 0;
                cfaOut[2] = 2;
                cfaOut[3] = 1;
                break;
            case 2:
                colorpattern = DngProfile.GBRG;
                cfaOut[0] = 1;
                cfaOut[1] = 2;
                cfaOut[2] = 0;
                cfaOut[3] = 1;
                break;
            case 3:
                colorpattern = DngProfile.BGGR;
                cfaOut[0] = 2;
                cfaOut[1] = 1;
                cfaOut[2] = 1;
                cfaOut[3] = 0;
                break;
            default:
                colorpattern = DngProfile.RGGB;
                cfaOut[0] = 0;
                cfaOut[1] = 1;
                cfaOut[2] = 1;
                cfaOut[3] = 2;
                break;
        }
        float[] color2;
        float[] color1;
        float[] neutral = new float[3];
        float[] forward2 = null;
        float[] forward1 = null;
        float[] reduction1 = null;
        float[] reduction2 = null;
        double[]finalnoise = null;
        if (customMatrix != null){
            color1 = customMatrix.ColorMatrix1;
            color2 = customMatrix.ColorMatrix2;
            neutral = customMatrix.NeutralMatrix;
            if (customMatrix.ForwardMatrix1 != null && customMatrix.ForwardMatrix1.length >0)
                forward1 = customMatrix.ForwardMatrix1;
            if (customMatrix.ForwardMatrix2 != null && customMatrix.ForwardMatrix2.length >0)
                forward2 = customMatrix.ForwardMatrix2;
            if (customMatrix.ReductionMatrix1 != null &&  customMatrix.ReductionMatrix1.length >0)
                reduction1 = customMatrix.ReductionMatrix1;
            if (customMatrix.ReductionMatrix2 != null && customMatrix.ReductionMatrix2.length >0)
                reduction2 = customMatrix.ReductionMatrix2;
            if (customMatrix.NoiseReductionMatrix != null && customMatrix.NoiseReductionMatrix.length >0)
                finalnoise = customMatrix.NoiseReductionMatrix;
        }
        else
        {
            color1 = getFloatMatrix(characteristics.get(CameraCharacteristics.SENSOR_COLOR_TRANSFORM1));
            color2 = getFloatMatrix(characteristics.get(CameraCharacteristics.SENSOR_COLOR_TRANSFORM2));
            Rational[] n = captureResult.get(CaptureResult.SENSOR_NEUTRAL_COLOR_POINT);
            neutral[0] = n[0].floatValue();
            neutral[1] = n[1].floatValue();
            neutral[2] = n[2].floatValue();
            forward2  = getFloatMatrix(characteristics.get(CameraCharacteristics.SENSOR_FORWARD_MATRIX2));
            //0.820300f, -0.218800f, 0.359400f, 0.343800f, 0.570300f,0.093800f, 0.015600f, -0.726600f, 1.539100f
            forward1  = getFloatMatrix(characteristics.get(CameraCharacteristics.SENSOR_FORWARD_MATRIX1));
            reduction1 = getFloatMatrix(characteristics.get(CameraCharacteristics.SENSOR_CALIBRATION_TRANSFORM1));
            reduction2 = getFloatMatrix(characteristics.get(CameraCharacteristics.SENSOR_CALIBRATION_TRANSFORM2));
            finalnoise = new double[6];
            getNoiseMatrix(cfaOut, finalnoise);

        }

        return DngProfile.getProfile(black,image.getWidth(), image.getHeight(),rawFormat, colorpattern, 0,
                color1,
                color2,
                neutral,
                forward1,
                forward2,
                reduction1,
                reduction2,
                finalnoise,
                ""
        );
    }

    private void getNoiseMatrix(int[] cfaOut, double[] finalnoise) {
        //noise
        Pair[] p = captureResult.get(CaptureResult.SENSOR_NOISE_PROFILE);
        double[] noiseys = new double[p.length*2];
        int i = 0;
        for (int h = 0; h < p.length; h++)
        {
            noiseys[i++] = (double)p[h].first;
            noiseys[i++] = (double)p[h].second;
        }
        double[] noise = new double[6];
        int[] cfaPlaneColor = {0, 1, 2};
        generateNoiseProfile(noiseys,cfaOut, cfaPlaneColor,3,noise);

        for (i = 0; i < noise.length; i++)
            if (noise[i] > 2 || noise[i] < -2)
                finalnoise[i] = 0;
            else
                finalnoise[i] = (float)noise[i];
        //noise end
    }

    private void generateNoiseProfile(double[] perChannelNoiseProfile, int[] cfa,
                                      int[] planeColors, int numPlanes,
        /*out*/double[] noiseProfile) {

        for (int p = 0; p < 3; ++p) {
            int S = p * 2;
            int O = p * 2 + 1;

            noiseProfile[S] = 0;
            noiseProfile[O] = 0;
            boolean uninitialized = true;
            for (int c = 0; c < 4; ++c) {
                if (cfa[c] == planeColors[p] && perChannelNoiseProfile[c * 2] > noiseProfile[S]) {
                    noiseProfile[S] = perChannelNoiseProfile[c * 2];
                    noiseProfile[O] = perChannelNoiseProfile[c * 2 + 1];
                    uninitialized = false;
                }
            }
            if (uninitialized) {
                Log.d(TAG, "%s: No valid NoiseProfile coefficients for color plane %zu");
            }
        }
    }

    private float[]getFloatMatrix(ColorSpaceTransform transform)
    {
        float[] ret = new float[9];
        ret[0] = roundTo6Places(transform.getElement(0, 0).floatValue());
        ret[1] = roundTo6Places(transform.getElement(1, 0).floatValue());
        ret[2] = roundTo6Places(transform.getElement(2, 0).floatValue());
        ret[3] = roundTo6Places(transform.getElement(0, 1).floatValue());
        ret[4] = roundTo6Places(transform.getElement(1, 1).floatValue());
        ret[5] = roundTo6Places(transform.getElement(2, 1).floatValue());
        ret[6] = roundTo6Places(transform.getElement(0, 2).floatValue());
        ret[7] = roundTo6Places(transform.getElement(1, 2).floatValue());
        ret[8] = roundTo6Places(transform.getElement(2, 2).floatValue());
        return ret;
    }

    private float roundTo6Places(float f )
    {
        return Math.round(f*1000000f)/1000000f;
    }


    private void logColorPattern()
    {
        int c= characteristics.get(CameraCharacteristics.SENSOR_INFO_COLOR_FILTER_ARRANGEMENT);
        switch (c)
        {
            case 1:
                Log.d(TAG, "ColorPattern: GRBG");
                break;
            case 2:
                Log.d(TAG, "ColorPattern: GBRG");
                break;
            case 3:
                Log.d(TAG, "ColorPattern: BGGR");
                break;
            default:
                Log.d(TAG, "ColorPattern: RGGB");
                break;
        }
    }

    private void logNeutralMatrix()
    {
        Rational[] n = captureResult.get(CaptureResult.SENSOR_NEUTRAL_COLOR_POINT);
        Log.d(TAG,"NeutralMatrix:" + n[0].floatValue() + ","+ n[1].floatValue()+","+n[2].floatValue());
    }

    private void logImageFormat(Image image)
    {
        switch (image.getFormat())
        {
            case ImageFormat.RAW10:
                Log.d(TAG,"ImageFormat:RAW10");
                break;
            case ImageFormat.RAW12:
                Log.d(TAG,"ImageFormat:RAW12");
                break;
            case ImageFormat.RAW_SENSOR:
                Log.d(TAG,"ImageFormat:RAW_SENSOR");
                break;
            case ImageFormat.JPEG:
                Log.d(TAG,"ImageFormat:JPEG");
                break;
        }
    }

}
