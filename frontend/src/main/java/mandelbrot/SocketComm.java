package mandelbrot;

import java.net.*;
import java.io.*;

public class SocketComm {

    // Socket comm variables
    private Socket mClientSocket;
    private PrintWriter mWriter;
    private BufferedReader mReader;

    public SocketComm(int port) throws Exception {
        // Connect to the backend
        mClientSocket = new Socket("127.0.0.1", port);
        mWriter = new PrintWriter(mClientSocket.getOutputStream(), true);
        mReader = new BufferedReader(new InputStreamReader(mClientSocket.getInputStream()));

        // Read a line
        String greeting = mReader.readLine();
        if (!greeting.equals("ready")) {
            throw new Exception("Error communicating with backend");
        }
    }

    public String sendAndReceive(String line) throws IOException {
        mWriter.println(line);
        return mReader.readLine();
    }

}
