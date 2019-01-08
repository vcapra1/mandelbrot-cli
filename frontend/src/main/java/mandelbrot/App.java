package mandelbrot;

import javafx.application.Application;
import javafx.stage.Stage;
import javafx.scene.Scene;

import javafx.scene.layout.VBox;

public class App extends Application {

    /**
     * SocketComm Instance used to communicate with the Rust
     * backend of the program
     */
    private SocketComm mSocketComm;

    /**
     * The two main panes that make up the GUI (render canvas
     * and the controls)
     */
    private ControlPane mControlsPane;
    private DisplayPane mDisplayPane;

    /**
     * Saved command-line arguments
     */
    private static String[] sArgs;

    public static void main(String[] args) {
        // Save the arguments for later access
        sArgs = args;

        // Launch the application
        launch(args);
    }

    @Override
    public void start(Stage primaryStage) {
        try {
            // Connect to the backend
            mSocketComm = new SocketComm(Integer.parseInt(sArgs[0]));
        } catch (Exception e) {
            e.printStackTrace();
            return;
        }

        // Create the DisplayPane
        mDisplayPane = new DisplayPane(800, 800);

        // Create the controls layout
        mControlsPane = new ControlPane(mSocketComm, gc, this);

        // Create the root layout
        VBox root = new VBox(mDisplayPane, mControlsPane);

        // Create and apply the scene
        Scene scene = new Scene(root, 800, 950);
        primaryStage.setScene(scene);

        // Show the window
        primaryStage.show();

        // Disable resizing the stage
        //primaryStage.setResizable(false);
    }

    /**
     * Get the DisplayPane instance to update the image
     */
    public DisplayPane getDisplayPane() {
        return mDisplayPane;
    }

    /**
     * Get the SocketComm instance to communicate with the Rust backend
     */
    public SocketComm getSocketComm() {
        return mSocketComm;
    }

    @Override
    public void stop() {
        try {
            // Close the socket and exit smoothly
            mSocketComm.close();
        } catch (Exception e) {}
    }

}
