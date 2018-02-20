package com.oztz.hackinglabmobile.adapter;

import android.content.Context;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.TextView;

import com.oztz.hackinglabmobile.R;
import com.oztz.hackinglabmobile.businessclasses.NavigationItem;

/**
 * Created by Tobi on 25.03.2015.
 */
public class NavigationAdapter extends ArrayAdapter {

    public NavigationAdapter(Context context, int resource, NavigationItem[] navigationItems) {
        super(context, resource, navigationItems);
    }

    @Override
    public View getView(int position, View convertView, ViewGroup parent) {
        View v = convertView;

        NavigationItem item = (NavigationItem)getItem(position);

        TextView title = null;
        if (v == null) {
            LayoutInflater inflater = LayoutInflater.from(getContext());

            if(item.type == NavigationItem.TYPE_ITEM){
                v = inflater.inflate(R.layout.item_navigation_item, null);
                title = (TextView) v.findViewById(R.id.navigation_item);
            }
            else {
                v = inflater.inflate(R.layout.item_navigation_title, null);
                v.setEnabled(false);
                v.setOnClickListener(null);
                title = (TextView) v.findViewById(R.id.navigation_title);
            }
        }

        if (item != null) {
            if (title != null) {
                title.setText(item.text);
            }
        }
        return v;
    }
}
