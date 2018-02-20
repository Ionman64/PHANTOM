package com.oztz.hackinglabmobile.activity;

import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.AsyncTask;
import android.os.Bundle;
import android.provider.Settings.Secure;
import android.text.Editable;
import android.text.TextWatcher;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;

import com.google.android.gms.gcm.GoogleCloudMessaging;
import com.google.gson.Gson;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.User;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.PostTask;

import java.io.IOException;

public class RegisterActivity extends Activity implements HttpResult {

    EditText nameEditText;
    TextView messageTextView;
    Button startButton;

    final static String PROJECT_NUMBER = "182393118726";
    private GoogleCloudMessaging gcm;
    private String regId, deviceId, userName;
    private User user;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        if(userExists()){
            chooseEvent();
        }
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_register);
        nameEditText = (EditText) findViewById(R.id.register_editText_name);
        messageTextView = (TextView) findViewById(R.id.register_username_message_textview);
        startButton = (Button) findViewById(R.id.register_button_start);
        startButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                getRegId();
            }
        });
        nameEditText.addTextChangedListener(new TextWatcher() {
            @Override
            public void beforeTextChanged(CharSequence s, int start, int count, int after) {

            }

            @Override
            public void onTextChanged(CharSequence s, int start, int before, int count) {

            }

            @Override
            public void afterTextChanged(Editable s) {
                if(!isUsernameValid(nameEditText.getText().toString())){
                    messageTextView.setText(getResources().getString(R.string.error_invalid_username));
                    startButton.setEnabled(false);
                } else{
                    messageTextView.setText("");
                    startButton.setEnabled(true);
                }
            }
        });

    }

    public void getRegId(){
        new AsyncTask<Void, Void, String>() {
            @Override
            protected String doInBackground(Void... params) {
                try {
                    if (gcm == null) {
                        gcm = GoogleCloudMessaging.getInstance(getApplicationContext());
                    }
                    regId = gcm.register(PROJECT_NUMBER);
                } catch (IOException ex) {
                    regId = "";
                }
                return regId;
            }
            @Override
            protected void onPostExecute(String msg) {
                if(msg.length() > 1){
                    postUser(msg);
                }
            }
        }.execute(null, null, null);
    }

    private boolean userExists(){
        SharedPreferences sharedPref = getSharedPreferences(
                getString(R.string.preferences_file), Context.MODE_PRIVATE);
        String username = sharedPref.getString("username", "");
        return !username.equals("");
    }

    private void saveUserData(){
        SharedPreferences sharedPref = getSharedPreferences(
                getString(R.string.preferences_file), Context.MODE_PRIVATE);
        SharedPreferences.Editor editor = sharedPref.edit();
        editor.putInt("userId", user.userID);
        editor.putString("username", userName);
        editor.putString("deviceId", deviceId);
        editor.commit();
    }

    private void postUser(String msg){
        deviceId = Secure.getString(getApplicationContext().getContentResolver(),
                Secure.ANDROID_ID);
        userName = nameEditText.getText().toString();
        user = new User(deviceId, userName, msg, 0);
        String jsonString = new Gson().toJson(user);

        Log.d("DEBUG", "POST DATA: " + jsonString);
        new PostTask(this).execute(getResources().getString(R.string.rootURL) + "user", jsonString);
    }

    private void chooseEvent(){
        Intent intent = new Intent(this, ChooseEventActivity.class);
        startActivity(intent);
        this.finish();
    }

    private boolean isUsernameValid(String userName){
        if(userName.matches("[a-zA-Z0-9_]{3,25}")){
            return true;
        }
        return false;
    }




    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(requestCode.equals("POST")){
            if(JsonString != null){
                try{
                    user = new Gson().fromJson(JsonString, User.class);
                    saveUserData();
                    chooseEvent();
                } catch(Exception e){
                    if(JsonString.contains("HTTP Status 422")){
                        messageTextView.setText(getResources().getString(R.string.error_username_already_exists));
                    } else {
                        messageTextView.setText(getResources().getString(R.string.error_unknown));
                    }
                }
            }
            else{
                messageTextView.setText(getResources().getString(R.string.error_no_server_connection));
            }
        }
    }
}
