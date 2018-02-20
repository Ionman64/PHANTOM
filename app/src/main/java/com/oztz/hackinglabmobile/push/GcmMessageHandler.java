package com.oztz.hackinglabmobile.push;


import android.app.IntentService;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.os.Handler;
import android.support.v4.app.NotificationCompat;
import android.support.v4.app.TaskStackBuilder;
import android.util.Log;

import com.google.android.gms.gcm.GoogleCloudMessaging;
import com.google.gson.Gson;
import com.oztz.hackinglabmobile.MainActivity;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.activity.VotingDetailActivity;
import com.oztz.hackinglabmobile.businessclasses.PushMessage;
import com.oztz.hackinglabmobile.businessclasses.Voting;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

public class GcmMessageHandler extends IntentService implements HttpResult {

    private Handler handler;
    private String message, title;
    private PushMessage pm;
    public GcmMessageHandler() {
        super("GcmMessageHandler");
    }

    @Override
    public void onCreate() {
        super.onCreate();
        handler = new Handler();
    }
    @Override
    protected void onHandleIntent(Intent intent) {
        Bundle extras = intent.getExtras();
        GoogleCloudMessaging gcm = GoogleCloudMessaging.getInstance(this);
        String messageType = gcm.getMessageType(intent);
        doNotification(extras);
        Log.i("GCM", "Received : (" +messageType+")  "+extras.getString("title"));
        GcmBroadcastReceiver.completeWakefulIntent(intent);

    }

    private void doNotification(Bundle extras){
        title = extras.getString("title");
        message = extras.getString("message");
        try{
            pm = new Gson().fromJson(message, PushMessage.class);
            if(title.equals("voting") && pm != null && App.username != null){
                new RequestTask(this).execute(getResources().getString(R.string.rootURL) + "voting/" +
                    String.valueOf(pm.votingID), "voting");
            } else if(title.equals("presentation_end")){
                //do nothing at the moment...
            } else {
                doNormalPushNotification(title, message);
            }
        } catch(Exception e){ }
    }

    private void doVotingNotification(String voting){
        String text = String.format(getResources().getString(R.string.voting_started), pm.name);

        NotificationCompat.Builder mBuilder =
                new NotificationCompat.Builder(this)
                        .setSmallIcon(R.drawable.ic_launcher)
                        .setContentTitle(getResources().getString(R.string.app_name))
                        .setContentText(text)
                        .setAutoCancel(true);

        Intent resultIntent = new Intent(this, VotingDetailActivity.class);
        resultIntent.putExtra("voting", voting);

        TaskStackBuilder stackBuilder = TaskStackBuilder.create(this);
        stackBuilder.addParentStack(MainActivity.class);
        stackBuilder.addNextIntent(resultIntent);
        PendingIntent resultPendingIntent =
                stackBuilder.getPendingIntent(
                        0,
                        PendingIntent.FLAG_UPDATE_CURRENT
                );
        mBuilder.setContentIntent(resultPendingIntent);
        NotificationManager mNotificationManager =
                (NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
// mId allows you to update the notification later on.
        mNotificationManager.notify(1, mBuilder.build());
    }

    private void doNormalPushNotification(String title, String message){
        NotificationCompat.Builder mBuilder =
                new NotificationCompat.Builder(this)
                        .setSmallIcon(R.drawable.ic_launcher)
                        .setContentTitle(title)
                        .setContentText(message)
                        .setAutoCancel(true);

        Intent resultIntent = new Intent(this, MainActivity.class);

        TaskStackBuilder stackBuilder = TaskStackBuilder.create(this);
        stackBuilder.addParentStack(MainActivity.class);
        stackBuilder.addNextIntent(resultIntent);
        PendingIntent resultPendingIntent =
                stackBuilder.getPendingIntent(
                        0,
                        PendingIntent.FLAG_UPDATE_CURRENT
                );
        mBuilder.setContentIntent(resultPendingIntent);
        NotificationManager mNotificationManager =
                (NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
// mId allows you to update the notification later on.
        mNotificationManager.notify(1, mBuilder.build());
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        if(requestCode.equals("voting")) {
            try {
                Voting voting = new Gson().fromJson(JsonString, Voting.class);
                SharedPreferences sharedPref = getSharedPreferences(
                        getString(R.string.preferences_file), Context.MODE_PRIVATE);
                int eventId = sharedPref.getInt("eventId", -1);
                if(eventId == voting.eventIDFK){
                    doVotingNotification(JsonString);
                }
            } catch (Exception e) {
                Log.d("DEBUG", e.getMessage());
            }
        }
    }
}
