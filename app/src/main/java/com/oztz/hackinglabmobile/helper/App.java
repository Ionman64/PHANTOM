package com.oztz.hackinglabmobile.helper;

import android.app.Application;
import android.content.Context;
import android.content.SharedPreferences;
import android.util.Log;

import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.database.DbOperator;

/**
 * Created by Tobi on 16.04.2015.
 */
public class App extends Application {
    private static Context mContext;
    public static String username;
    public static String deviceId;
    public static int userId;
    public static int eventId;
    public static int newestSelectLimit;
    public static DbOperator db; //makes all operations on Database

    @Override
    public void onCreate() {
        super.onCreate();
        mContext = getApplicationContext();
    }

    public static void loadVariables(){
        SharedPreferences sharedPref = mContext.getSharedPreferences(
                mContext.getString(R.string.preferences_file), Context.MODE_PRIVATE);
        username = sharedPref.getString("username", "");
        userId = sharedPref.getInt("userId", 0);
        deviceId = sharedPref.getString("deviceId", "");
        eventId = sharedPref.getInt("eventId", 0);
        newestSelectLimit = sharedPref.getInt("newestSelectLimit", 15);
        db = new DbOperator(mContext);
        Log.d("DEBUG", "username=" + username + "; userId=" + String.valueOf(userId) + "; " +
                "deviceId=" + String.valueOf(deviceId));
    }

    public static Context getContext(){
        return mContext;
    }
}
