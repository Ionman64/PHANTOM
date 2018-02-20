package com.oztz.hackinglabmobile.helper;

import android.os.AsyncTask;
import android.util.Log;

/**
 * Created by Tobi on 25.03.2015.
 */
public class RequestTask extends AsyncTask<String, String, String> {
    private HttpResult listener;
    String result;
    String requestCode;

    public RequestTask(HttpResult listener){
        this.listener = listener;
    }

    @Override
    protected String doInBackground(String... uri) {
        if(uri.length > 1){
            requestCode = uri[1];
        }
        try {
            Log.d("DEBUG", "Read " + uri[0]);
            result = new HttpHelper().readUrl(uri[0]);
        } catch (Exception e){
            result = null;
        }

        if(result != null && App.db != null){
            App.db.saveToDataBase(uri[0], result);
        }
        return result;
    }

    @Override
    protected void onPostExecute(String result) {
        super.onPostExecute(result);

        listener.onTaskCompleted(result, requestCode);
    }
}