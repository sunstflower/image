import React from "react";
import { Toaster } from "react-hot-toast";
import { ConversionPage } from "./components/ConversionPage";
import "./App.css";

function App() {
  return (
    <div className="App">
      <ConversionPage />
      <Toaster
        position="top-right"
        toastOptions={{
          duration: 4000,
          style: {
            background: "#363636",
            color: "#fff",
          },
          success: {
            duration: 3000,
            iconTheme: {
              primary: "#4ade80",
              secondary: "#fff",
            },
          },
          error: {
            duration: 5000,
            iconTheme: {
              primary: "#ef4444",
              secondary: "#fff",
            },
          },
        }}
      />
    </div>
  );
}

export default App;
