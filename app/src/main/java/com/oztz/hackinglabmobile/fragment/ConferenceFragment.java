package com.oztz.hackinglabmobile.fragment;

import android.app.Activity;
import android.os.Bundle;
import android.support.v4.app.Fragment;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ImageView;
import android.widget.TextView;
import android.widget.Toast;

import com.google.gson.Gson;
import com.nostra13.universalimageloader.core.DisplayImageOptions;
import com.nostra13.universalimageloader.core.ImageLoader;
import com.nostra13.universalimageloader.core.ImageLoaderConfiguration;
import com.oztz.hackinglabmobile.MainActivity;
import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.Event;
import com.oztz.hackinglabmobile.helper.App;
import com.oztz.hackinglabmobile.helper.AuthImageDownloader;
import com.oztz.hackinglabmobile.helper.HttpResult;
import com.oztz.hackinglabmobile.helper.RequestTask;

/**
 * Created by Tobi on 20.03.2015.
 */
public class ConferenceFragment extends Fragment implements HttpResult {

    private static final String ARG_SECTION_NUMBER = "section_number";
    private TextView titleTextView, descriptionTextView;
    private ImageView eventPicture;
    private ImageLoader imageLoader;



    public static ConferenceFragment newInstance(int sectionNumber) {
        Log.d("DEBUG", "ConferenceFragment.newInstance(" + String.valueOf(sectionNumber) + ")");
        ConferenceFragment fragment = new ConferenceFragment();
        Bundle args = new Bundle();
        args.putInt(ARG_SECTION_NUMBER, sectionNumber);
        fragment.setArguments(args);
        return fragment;
    }

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState)
    {
        View view = inflater.inflate(R.layout.fragment_conference, container, false);
        titleTextView = (TextView)view.findViewById(R.id.conference_title);
        eventPicture= (ImageView) view.findViewById(R.id.event_picture);
        descriptionTextView = (TextView)view.findViewById(R.id.conference_text);
        String url = getResources().getString(R.string.rootURL) + "event/" +
                String.valueOf(App.eventId);
        updateView(App.db.getContentFromDataBase(url));
        new RequestTask(this).execute(url, "event");
        return view;
    }

    @Override
    public void onAttach(Activity activity) {
        super.onAttach(activity);
        ((MainActivity) activity).onSectionAttached(getArguments().getInt(
                ARG_SECTION_NUMBER));
    }

    private void updateView(String JsonString){
        try {
            Event event = new Gson().fromJson(JsonString, Event.class);
            titleTextView.setText(event.name);
            descriptionTextView.setText(event.description);
            if(event.media != null && event.media.length() > 1){
                imageLoader = ImageLoader.getInstance();
                ImageLoaderConfiguration config = new ImageLoaderConfiguration.Builder(getActivity().getApplicationContext())
                        .imageDownloader(new AuthImageDownloader(getActivity().getApplicationContext(), 5000, 20000))
                        .diskCacheFileCount(50)
                        .defaultDisplayImageOptions(new DisplayImageOptions.Builder()
                                .cacheInMemory(true)
                                .cacheOnDisk(true).build())
                        .build();
                imageLoader.init(config);
                imageLoader.displayImage(event.media, eventPicture);
            }

        } catch(Exception e){
            Toast.makeText(getActivity(), "Error loading data", Toast.LENGTH_SHORT);
        }
    }

    @Override
    public void onTaskCompleted(String JsonString, String requestCode) {
        updateView(JsonString);
    }
}
