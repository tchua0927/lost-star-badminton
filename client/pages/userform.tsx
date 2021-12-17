import React, { useState } from 'react'
import '../style/form.css'

export default function UserForm() {
    
    const [term, setTerm] = useState("");
    const submitForm = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        alert(event);
    }

    return (
        <div className="container">
            <form action="onSubmit">submitForm</form>
            <input
                value={term}
                onChange={(e) => setTerm(e.target.value)}
                type="text"
                placeholder="Enter a term"
                className="input"
            />
            <button type="submit" className="btn">Submit</button>
        </div>
    );
    
}