package com.oztz.hackinglabmobile.helper;

import android.os.AsyncTask;

/**
 * Created by Tobi on 25.03.2015.
 */
public class PostMediaTask extends AsyncTask<String, String, String> {
    private HttpResult listener;
    String result;

    public PostMediaTask(HttpResult listener){
        this.listener = listener;
    }

    @Override
    protected String doInBackground(String... uri) {
        try {
            result = new HttpHelper().PostMedia(uri[0], uri[1]);
        } catch (Exception e){
            result = null;
        }
        return result;
    }

    @Override
    protected void onPostExecute(String result) {
        super.onPostExecute(result);
        listener.onTaskCompleted(result, "POST_MEDIA");
    }
}