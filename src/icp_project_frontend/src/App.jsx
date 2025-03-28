import { useState } from 'react';
import { icp_project_backend } from '../../declarations/icp_project_backend';

function App() {
  const [inputValue, setInputValue] = useState('');
  
  async function handleSubmit(event) {
    event.preventDefault();
    const number = event.target.elements.name.value;
    await icp_project_backend.add_value(BigInt(number));
    await icp_project_backend.get_value().then(sum => {
      setInputValue(Number(sum))
    });
    return false;
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your Number: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
      </form>
      <section id="greeting">Totale : {inputValue}</section>
    </main>
  );
}

export default App;
