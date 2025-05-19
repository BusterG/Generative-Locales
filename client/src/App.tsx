import React, { useState } from "react";

function App() {
  const [count, setCount] = useState<number | null>(null);

  const getCount = async () => {
    alert("Simulated call to Stellar contract");
    setCount((prev) => (prev ?? 0) + 1);
  };

  return (
    <div style={{ padding: 20 }}>
      <h1>Hello Counter</h1>
      <button onClick={getCount}>Increment</button>
      <p>Count: {count}</p>
    </div>
  );
}

export default App;
