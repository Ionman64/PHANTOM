package com.oztz.hackinglabmobile.helper;

import android.content.Context;
import android.util.Log;

import com.nostra13.universalimageloader.core.download.BaseImageDownloader;

import java.io.IOException;
import java.io.InputStream;

/**
 * Created by Tobi on 16.04.2015.
 */
public class AuthImageDownloader extends BaseImageDownloader {
    public static final String TAG = AuthImageDownloader.class.getName();


    public AuthImageDownloader(Context context, int connectTimeout, int readTimeout) {
        super(context, connectTimeout, readTimeout);
    }

    @Override
    protected InputStream getStreamFromNetwork(String imageUri, Object extra) throws IOException {
        Log.d("DEBUG", "Download Image from " + imageUri);
        return new HttpHelper().getURLConnection(imageUri);
    }
}
