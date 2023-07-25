import './App.css'
import { useEffect, useState } from 'react'
import { listen, emit } from '@tauri-apps/api/event'
// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event


// await invoke("initialize");

function App() {
  const [keyHistory, setKeyHistory] = useState<string[]>([])

  useEffect(() => {
    listen('key_event', (event) => {
      let { key } = event.payload as any;
      setKeyHistory(prev => {
        if (prev[prev.length - 1] === key) return prev;

        return [...prev, key]
      })
    })
  }, [])

  return (
    <>
      <div className="keyscroll-container">
        {keyHistory}
      </div>
    </>
  )
}

export default App
