package com.troop.menu;

import android.view.MenuItem;
import android.view.View;
import android.widget.PopupMenu;

import com.troop.freecam.CameraManager;
import com.troop.freecam.MainActivity;
import com.troop.freecam.manager.ParametersManager;

import java.util.List;

/**
 * Created by troop on 29.08.13.
 */
public class SceneMenu extends BaseMenu
{
    List<String> sceneModes;
    public SceneMenu(CameraManager camMan, MainActivity activity) {
        super(camMan, activity);
    }

    @Override
    public void onClick(View v)
    {
        if(camMan.Running)
            sceneModes = camMan.parametersManager.getParameters().getSupportedSceneModes();
        if (sceneModes != null)
        {
            PopupMenu popupMenu = new PopupMenu(activity, super.GetPlaceHolder());
            //popupMenu.getMenuInflater().inflate(R.menu.menu_popup_flash, popupMenu.getMenu().);
            for (int i = 0; i < sceneModes.size(); i++) {
                popupMenu.getMenu().add((CharSequence) sceneModes.get(i));
            }
            popupMenu.setOnMenuItemClickListener(new PopupMenu.OnMenuItemClickListener() {

                @Override
                public boolean onMenuItemClick(MenuItem item) {
                    String tmp = item.toString();
                    String camvalue = preferences.getString(ParametersManager.SwitchCamera, ParametersManager.SwitchCamera_MODE_3D);
                    if (camvalue.equals(ParametersManager.SwitchCamera_MODE_3D))
                        preferences.edit().putString(ParametersManager.Preferences_Scene3D, tmp).commit();
                    if (camvalue.equals(ParametersManager.SwitchCamera_MODE_2D))
                        preferences.edit().putString(ParametersManager.Preferences_Scene2D, tmp).commit();
                    if (camvalue.equals(ParametersManager.SwitchCamera_MODE_Front))
                        preferences.edit().putString(ParametersManager.Preferences_SceneFront, tmp).commit();
                    //preferences.edit().putString("scene", tmp).commit();
                    camMan.parametersManager.getParameters().setSceneMode(tmp);

                    camMan.Restart(false);

                    return true;
                }
            });

            popupMenu.show();
        }

    }
}
