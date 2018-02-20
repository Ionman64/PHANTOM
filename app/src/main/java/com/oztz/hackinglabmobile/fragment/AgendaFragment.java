package com.oztz.hackinglabmobile.fragment;

import android.content.Intent;
import android.graphics.Color;
import android.graphics.RectF;
import android.os.Bundle;
import android.support.v4.app.Fragment;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Toast;

import com.alamkanak.weekview.WeekView;
import com.alamkanak.weekview.WeekViewEvent;
import com.google.gson.Gson;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.activity.EventItemDetailActivity;
import com.oztz.hackinglabmobile.businessclasses.EventItem;
import com.oztz.hackinglabmobile.businessclasses.EventRoom;

import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Calendar;
import java.util.Date;
import java.util.List;

/**
 * Created by Tobi on 20.03.2015.
 */
public class AgendaFragment extends Fragment implements WeekView.MonthChangeListener,
        WeekView.EventClickListener, WeekView.EventLongPressListener {

    //private static final int[] roomColors = {Color.BLUE, Color.GREEN, Color.RED, Color.YELLOW, Color.CYAN};
    WeekView mWeekView;
    private List<WeekViewEvent> eventList;
    //private HashSet rooms;
    EventItem[] eventItems = null;
    EventRoom[] eventRooms = null;

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        eventList = new ArrayList<WeekViewEvent>();
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState)
    {
        View view = inflater.inflate(R.layout.fragment_agenda, container, false);
        // Get a reference for the week view in the layout.
        mWeekView = (WeekView) view.findViewById(R.id.weekView);
        mWeekView.setOnEventClickListener(this);
        mWeekView.setMonthChangeListener(this);
        mWeekView.setEventLongPressListener(this);
        String itemsString = this.getArguments().getString("eventitems", "");
        String roomsString = this.getArguments().getString("rooms", "");
        loadItems(itemsString, roomsString);

        goToFirstItem();
        return view;
    }

    public void loadItems(String itemsString, String roomsString) {
        try {
            eventItems = new Gson().fromJson(itemsString, EventItem[].class);
            eventRooms = new Gson().fromJson(roomsString, EventRoom[].class);
            eventList.clear();
            for(int i=0;i<eventItems.length;i++){
                WeekViewEvent e = getEvent(eventItems[i]);
                eventList.add(e);
                Log.d("DEBUG", e.getName() + " added");
            }
            mWeekView.notifyDatasetChanged();
            Log.d("DEBUG", "mWeekView.notifyDatasetChanged()");
        } catch(Exception e){
            e.printStackTrace();
        }
    }

    private void goToFirstItem(){
        if(eventItems != null && eventItems.length > 0){
            try {
                SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm");
                Date earliest = sdf.parse(eventItems[0].date + " " + eventItems[0].startTime);
                for (int i = 1; i < eventItems.length; i++) {
                    EventItem item = eventItems[i];
                    Date date = sdf.parse(item.date + " " + item.startTime);
                    if(date.before(earliest)){
                        earliest = date;
                    }
                }
                Calendar c = Calendar.getInstance();
                c.setTime(earliest);
                mWeekView.goToDate(c);
                mWeekView.goToHour(earliest.getHours());
                Log.d("DEBUG", "Go To " + sdf.format(c.getTime()));
            } catch (Exception e){
                mWeekView.goToHour(Calendar.getInstance().get(Calendar.HOUR_OF_DAY));
            }
        } else {
            mWeekView.goToHour(Calendar.getInstance().get(Calendar.HOUR_OF_DAY));
        }
    }

    @Override
    public void onEventClick(WeekViewEvent weekViewEvent, RectF rectF) {
        Intent intent = new Intent(getActivity(), EventItemDetailActivity.class);
        EventItem item = getEventItem(weekViewEvent.getId());
        intent.putExtra("eventItem", new Gson().toJson(item, EventItem.class));
        intent.putExtra("eventRoom", new Gson().toJson(getEventRoom(item.roomIDFK)));
        startActivity(intent);
    }

    @Override
    public void onEventLongPress(WeekViewEvent weekViewEvent, RectF rectF) {

    }

    @Override
    public List<WeekViewEvent> onMonthChange(int newYear, int newMonth) {
        List<WeekViewEvent> events = new ArrayList<WeekViewEvent>();
        for(int i=0; i<eventList.size();i++){
            WeekViewEvent w = eventList.get(i);
            if(w.getStartTime().get(Calendar.MONTH) == newMonth-1){
                events.add(w);
            }
        }
        return events;
    }

    private String getColor(int roomID){
        for(int i=0; i< eventRooms.length; i++){
            if(roomID == eventRooms[i].eventRoomID){
                return eventRooms[i].color;
            }
        }
        return eventRooms[0].color;
    }

    private EventItem getEventItem(long id){
        for(int i=0; i<eventItems.length; i++){
            if(eventItems[i].eventItemID == id){
                return eventItems[i];
            }
        }
        return null;
    }

    private EventRoom getEventRoom(int id){
        for(int i=0; i<eventRooms.length; i++){
            if(eventRooms[i].eventRoomID == id){
                return eventRooms[i];
            }
        }
        return null;
    }

    private WeekViewEvent getEvent(EventItem item){
        Calendar startTime = Calendar.getInstance();
        Calendar endTime = (Calendar) startTime.clone();
        try {
            startTime.setTime(new SimpleDateFormat("yyyy-MM-dd HH:mm").parse(item.date + " " + item.startTime));
            endTime.setTime(new SimpleDateFormat("yyyy-MM-dd HH:mm").parse(item.date + " " + item.endTime));
            WeekViewEvent event = new WeekViewEvent(item.eventItemID, item.name, startTime, endTime);
            event.setColor(Color.parseColor(getColor(item.roomIDFK)));
            return event;
        } catch(Exception e){
            Toast.makeText(getActivity(), "Error Parsing Dates", Toast.LENGTH_SHORT).show();
        }
        return null;
    }
}
