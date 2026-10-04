import { useEffect } from "react";
import { subscribeToSnapshots } from "../services/backend";
import { useDeviceStore } from "../stores/deviceStore";

export function useBackendEvents() {
  const applySnapshot = useDeviceStore((state) => state.applySnapshot);

  useEffect(() => {
    let disposed = false;
    let unsubscribe: (() => void) | undefined;

    void subscribeToSnapshots((snapshot) => {
      if (!disposed) {
        applySnapshot(snapshot);
      }
    }).then((cleanup) => {
      unsubscribe = cleanup;
      if (disposed) {
        cleanup();
      }
    });

    return () => {
      disposed = true;
      unsubscribe?.();
    };
  }, [applySnapshot]);
}

