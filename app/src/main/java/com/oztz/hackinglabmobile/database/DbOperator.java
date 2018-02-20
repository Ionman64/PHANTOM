package com.oztz.hackinglabmobile.database;

import android.content.ContentValues;
import android.content.Context;
import android.database.Cursor;
import android.database.sqlite.SQLiteDatabase;
import android.util.Log;

/**
 * Created by Tobi on 08.05.2015.
 */
public class DbOperator {

    HackingLabDbHelper helper;

    public DbOperator(Context context){
        helper = new HackingLabDbHelper(context);
    }

    public boolean addQrCode(String Payload){
        try {
            String[] parts = Payload.split("-");
            if (parts.length == 3) {
                SQLiteDatabase db = helper.getWritableDatabase();
                String query = "INSERT INTO " + HackingLabDbHelper.QR_TABLE_NAME + "" +
                        "(role, eventid, secret) VALUES ('" +
                        parts[0] + "', " + parts[1] + ", '" + parts[2] + "');";
                db.execSQL(query);
                Log.d("DEBUG", query);
                return true;
            }
            else{
                return false;
            }
        } catch(Exception e){
            Log.d("DEBUG", e.getMessage());
            return false;
        }
    }

    public String getQrCode(String role, int eventid){
        SQLiteDatabase db = helper.getReadableDatabase();
        String query = "SELECT * FROM " + HackingLabDbHelper.QR_TABLE_NAME +
                " WHERE role = '" + role + "' AND eventid = " + String.valueOf(eventid);
        Cursor c = db.rawQuery(query, null);
        if(c.getCount() > 0){
            c.moveToFirst();
            return c.getString(c.getColumnIndex("role")) + "-" +
                    c.getInt(c.getColumnIndex("eventid")) + "-" +
                    c.getString(c.getColumnIndex("secret"));
        } else{
            return null;
        }
    }

    public boolean saveToDataBase(String key, String json){
        try {
            SQLiteDatabase db = helper.getWritableDatabase();
            ContentValues cv = new ContentValues();
            cv.put("content", json);

            String content = getContentFromDataBase(key);
            String query;
            if (content != null) {
                db.update(HackingLabDbHelper.CONTENT_TABLE_NAME,cv, "key = ?", new String[]{key});
                query = "UPDATE " + HackingLabDbHelper.CONTENT_TABLE_NAME +
                        " SET content = '" + json + "' WHERE key='" + key + "'";
            } else {
                cv.put("key", key);
                db.insert(HackingLabDbHelper.CONTENT_TABLE_NAME, null, cv);
                query = "INSERT INTO " + HackingLabDbHelper.CONTENT_TABLE_NAME + "" +
                        "(key, content) VALUES ('" +
                        key + "', '" + json + "');";
            }
            //db.execSQL(query);
            Log.d("DEBUG", query);
            return true;
        } catch(Exception e){
            Log.d("DEBUG", e.getMessage());
            return false;
        }

    }

    public String getContentFromDataBase(String key){
        SQLiteDatabase db = helper.getReadableDatabase();
        String query = "SELECT * FROM " + HackingLabDbHelper.CONTENT_TABLE_NAME +
                " WHERE key = '" + key + "'";
        Cursor c = db.query(HackingLabDbHelper.CONTENT_TABLE_NAME,
                new String[]{"key", "content"},
                "key=?", new String[] { key }, null, null, null);
        //Cursor c = db.rawQuery(query, null);
        if(c.getCount() > 0){
            c.moveToFirst();
            return c.getString(c.getColumnIndex("content"));
        } else{
            return null;
        }
    }
}
