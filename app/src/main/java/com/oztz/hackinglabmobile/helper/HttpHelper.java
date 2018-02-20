package com.oztz.hackinglabmobile.helper;

import android.content.Context;
import android.content.SharedPreferences;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.util.Base64;
import android.util.Log;

import com.oztz.hackinglabmobile.R;

import org.apache.http.HttpEntity;
import org.apache.http.HttpResponse;
import org.apache.http.HttpVersion;
import org.apache.http.client.methods.HttpPost;
import org.apache.http.conn.ClientConnectionManager;
import org.apache.http.conn.scheme.PlainSocketFactory;
import org.apache.http.conn.scheme.Scheme;
import org.apache.http.conn.scheme.SchemeRegistry;
import org.apache.http.conn.ssl.SSLSocketFactory;
import org.apache.http.entity.ContentType;
import org.apache.http.entity.StringEntity;
import org.apache.http.entity.mime.HttpMultipartMode;
import org.apache.http.entity.mime.MultipartEntityBuilder;
import org.apache.http.entity.mime.content.ByteArrayBody;
import org.apache.http.impl.client.DefaultHttpClient;
import org.apache.http.impl.conn.tsccm.ThreadSafeClientConnManager;
import org.apache.http.params.BasicHttpParams;
import org.apache.http.params.HttpParams;
import org.apache.http.params.HttpProtocolParams;
import org.apache.http.protocol.HTTP;
import org.apache.http.util.EntityUtils;

import java.io.BufferedInputStream;
import java.io.BufferedReader;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.UnsupportedEncodingException;
import java.net.HttpURLConnection;
import java.net.URL;
import java.security.KeyStore;
import java.security.cert.Certificate;
import java.security.cert.CertificateFactory;
import java.security.cert.X509Certificate;

import javax.net.ssl.HttpsURLConnection;
import javax.net.ssl.SSLContext;
import javax.net.ssl.TrustManagerFactory;

/**
 * Created by Tobi on 20.03.2015.
 */
public class HttpHelper {

    /*public void getWebsite(String url){
        new RequestTask().execute(url);
    }*/

    public String readUrl(String urlString) throws IOException {
        InputStream inStream = null;
        try{
            inStream = getURLConnection(urlString);
            String result = readIt(inStream);
            Log.d("DEBUG", result);
            return result;
        } finally {
            if(inStream != null){
                inStream.close();
            }
        }
    }

    private String readIt(InputStream stream) throws IOException, UnsupportedEncodingException {
        StringBuilder inputStringBuilder = new StringBuilder();
        BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(stream, "UTF-8"));
        String line = bufferedReader.readLine();
        while(line != null){
            inputStringBuilder.append(line);inputStringBuilder.append('\n');
            line = bufferedReader.readLine();
        }
        return inputStringBuilder.toString();
    }

    public InputStream getURLConnection(String urlString){
        try {
            URL url = new URL(urlString);
            String scheme = url.toURI().getScheme();
            if(scheme.equals("http")){
                return getHTTPConnection(url);
            } else if(scheme.equals("https")){
                return getHTTPSConnection(url);
            } else{
                return null;
            }
        } catch (Exception e){
            return null;
        }
    }

    public InputStream getHTTPConnection(URL url){
        try{
            HttpURLConnection urlConnection = (HttpURLConnection) url.openConnection();
            urlConnection.setReadTimeout(5000);
            urlConnection.setConnectTimeout(5000);
            urlConnection.setRequestMethod("GET");
            urlConnection.setDoInput(true);
            InputStream in = urlConnection.getInputStream();
            return in;
        } catch (Exception e){
            e.printStackTrace();
            return null;
        }
    }

    public InputStream getHTTPSConnection(URL url){
        try {
            CertificateFactory cf = CertificateFactory.getInstance("X.509");
            InputStream caInput = new BufferedInputStream(App.getContext().getAssets().open("hlmng.crt"));
            Certificate ca;
            try {
                ca = cf.generateCertificate(caInput);
                System.out.println("ca=" + ((X509Certificate) ca).getSubjectDN());
            } finally {
                caInput.close();
            }

            String keyStoreType = KeyStore.getDefaultType();
            KeyStore keyStore = KeyStore.getInstance(keyStoreType);
            keyStore.load(null, null);
            keyStore.setCertificateEntry("ca", ca);

            String tmfAlgorithm = TrustManagerFactory.getDefaultAlgorithm();
            TrustManagerFactory tmf = TrustManagerFactory.getInstance(tmfAlgorithm);
            tmf.init(keyStore);

            SSLContext context = SSLContext.getInstance("TLS");
            context.init(null, tmf.getTrustManagers(), null);

            HttpsURLConnection urlConnection =
                    (HttpsURLConnection) url.openConnection();
            urlConnection.setSSLSocketFactory(context.getSocketFactory());
            urlConnection.setHostnameVerifier(org.apache.http.conn.ssl.SSLSocketFactory.ALLOW_ALL_HOSTNAME_VERIFIER);
            urlConnection.setReadTimeout(5000);
            urlConnection.setConnectTimeout(5000);
            urlConnection.setRequestMethod("GET");
            urlConnection.setDoInput(true);
            InputStream in = urlConnection.getInputStream();
            return in;
        } catch (Exception e){
            e.printStackTrace();
            return null;
        }
    }

    public String doPost(String urlString, String jsonData){
        DefaultHttpClient httpClient = createHttpClient();
        HttpPost httpPost = new HttpPost(urlString);
        httpPost.addHeader("Authorization", getAuthHeader());
        StringEntity se;
        try {
            se = new StringEntity(jsonData);
            se.setContentType("application/json;charset=UTF-8");
            httpPost.setEntity(se);
            HttpResponse response = httpClient.execute(httpPost);
            String result = EntityUtils.toString(response.getEntity());
            Log.d("DEBUG", result);
            return result;
        } catch (Exception e) {
            e.printStackTrace();
            return null;
        }
    }

    public String doPost(String urlString, String jsonData, String header){
        DefaultHttpClient httpClient = createHttpClient();
        HttpPost httpPost = new HttpPost(urlString);
        httpPost.addHeader("Authorization", getAuthHeader());
        httpPost.addHeader("X-QRCode", header);
        StringEntity se;
        try {
            se = new StringEntity(jsonData);
            se.setContentType("application/json;charset=UTF-8");
            httpPost.setEntity(se);
            HttpResponse response = httpClient.execute(httpPost);
            String result = EntityUtils.toString(response.getEntity());
            Log.d("DEBUG", result);
            return result;
        } catch (Exception e) {
            e.printStackTrace();
            return "ERROR";
        }
    }

    public String PostMedia(String serverUrl, String mediaPath){
        DefaultHttpClient httpClient = createHttpClient();
        HttpPost httpPost = new HttpPost(serverUrl);
        httpPost.addHeader("Authorization", getAuthHeader());
        MultipartEntityBuilder builder = MultipartEntityBuilder.create();
        builder.setMode(HttpMultipartMode.BROWSER_COMPATIBLE);
        ByteArrayBody bab = getCompressedImage(mediaPath);
        builder.addPart("file", bab);
        final HttpEntity entity = builder.build();
        httpPost.setEntity(entity);
        try {
            HttpResponse response = httpClient.execute(httpPost);
            String resultString = EntityUtils.toString(response.getEntity());
            Log.d("DEBUG", resultString);
            return resultString;
        } catch (Exception e){
            e.printStackTrace();
        }
        return "";
    }

    private ByteArrayBody getCompressedImage(String mediaPath){
        File file = new File(mediaPath);
        Bitmap source = BitmapFactory.decodeFile(mediaPath);

        double sourceWidth = source.getWidth();
        double sourceHeight = source.getHeight();
        if(sourceWidth > 1000 || sourceHeight > 1000){
            double max = Math.max(sourceWidth, sourceHeight);
            double scaleFactor = 1000 / max;
            int scaledWidth = (int)(sourceWidth * scaleFactor);
            int scaledHeight = (int)(sourceHeight * scaleFactor);
            source = Bitmap.createScaledBitmap(source, scaledWidth, scaledHeight, true);
        }

        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        source.compress(Bitmap.CompressFormat.JPEG, 60, baos);
        byte[] imageBytes = baos.toByteArray();
        String filename = String.valueOf(System.currentTimeMillis()) + ".jpg";
        return new ByteArrayBody(imageBytes, ContentType.create(getContentType(file.getName())), filename);
    }

    private String getContentType(String fileName){
        String[] parts = fileName.split("\\.");
        String extension = parts[parts.length - 1].toLowerCase();
        if(extension.equals("jpg") || extension.equals("jpeg")){
            return "image/jpeg";
        }
        else if(extension.equals("png")){
            return "image/png";
        }
        return "";
    }

    private String getAuthHeader(){
        SharedPreferences sharedPref = App.getContext().getSharedPreferences(
                App.getContext().getString(R.string.preferences_file), Context.MODE_PRIVATE);
        String user = sharedPref.getString("username", "");
        String deviceId = sharedPref.getString("deviceId", "");

        String source = user + ":" + deviceId;
        return "Basic "+ Base64.encodeToString(source.getBytes(), Base64.URL_SAFE|Base64.NO_WRAP);
    }

    private DefaultHttpClient createHttpClient()
    {
        try {
            KeyStore trustStore = KeyStore.getInstance(KeyStore.getDefaultType());
            trustStore.load(null, null);

            MySSLSocketFactory sf = new MySSLSocketFactory(trustStore);
            sf.setHostnameVerifier(SSLSocketFactory.ALLOW_ALL_HOSTNAME_VERIFIER);

            HttpParams params = new BasicHttpParams();
            HttpProtocolParams.setVersion(params, HttpVersion.HTTP_1_1);
            HttpProtocolParams.setContentCharset(params, HTTP.UTF_8);

            SchemeRegistry registry = new SchemeRegistry();
            registry.register(new Scheme("http", PlainSocketFactory.getSocketFactory(), 80));
            registry.register(new Scheme("https", sf, 443));

            ClientConnectionManager ccm = new ThreadSafeClientConnManager(params, registry);

            return new DefaultHttpClient(ccm, params);
        } catch (Exception e) {
            return new DefaultHttpClient();
        }
    }


}


