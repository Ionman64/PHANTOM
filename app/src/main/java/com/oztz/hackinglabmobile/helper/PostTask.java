package com.oztz.hackinglabmobile.helper;

import android.os.AsyncTask;

/**
 * Created by Tobi on 25.03.2015.
 */
public class PostTask extends AsyncTask<String, String, String> {
    private HttpResult listener;
    String result;

    public PostTask(HttpResult listener){
        this.listener = listener;
    }

    @Override
    protected String doInBackground(String... uri) {
        try {
            if(uri.length == 2) {
                result = new HttpHelper().doPost(uri[0], uri[1]); //URL & JSONData
            } else if(uri.length == 3){
                result = new HttpHelper().doPost(uri[0], uri[1], uri[2]); // Contains QR-Code
            }
        } catch (Exception e){
            result = null;
        }
        return result;
    }

    @Override
    protected void onPostExecute(String result) {
        super.onPostExecute(result);
        listener.onTaskCompleted(result, "POST");
    }
}