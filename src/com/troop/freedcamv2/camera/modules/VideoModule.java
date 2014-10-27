package com.troop.freedcamv2.camera.modules;

import com.troop.freedcamv2.camera.BaseCameraHolder;
import com.troop.freedcamv2.ui.AppSettingsManager;

/**
 * Created by troop on 16.08.2014.
 */
public class VideoModule extends AbstractModule
{
    public final String TAG = "freedcam.VideoModule";

    public VideoModule(BaseCameraHolder cameraHandler, AppSettingsManager Settings, ModuleEventHandler eventHandler) {
        super(cameraHandler, Settings, eventHandler);
        name  = ModuleHandler.MODULE_VIDEO;
    }


//I_Module START
    @Override
    public String ModuleName() {
        return name;
    }

    @Override
    public void DoWork()
    {
        if (!isWorking)
            startRecording();
        else
            stopRecording();

    }

    @Override
    public boolean IsWorking() {
        return isWorking;
    }
//I_Module END


    private void startRecording()
    {
        //TODO add recording logic
    }

    private void stopRecording()
    {
        //TODO add stop recording logic
    }
}
