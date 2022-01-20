import React from 'react';


const Name = (props) => {
    return(
        <div>
            {props.fname+ " " + props.lname}
        </div>
    );
}

const Username = (props) => {

    return props.username? (
        <div>
            {props.username}
        </div>
    ):(<div></div>)
}

const Contact = (props) => {
    return(
        <div>
            {props.contact.email}
            <br />
            {props.contact.phone && props.contact.phone}
            
        </div>
    );

    
}


const Membership = (props) => {
    if (!props.membership) 
        return (<div></div>);
    
    const start = new Date(Number.parseInt(props.membership.start['$date']['$numberLong'])).toLocaleDateString();
    let end = null;
    if (props.membership.exp)
        end = new Date(Number.parseInt(props.membership.stop['$date']['$numberLong'])).toLocaleDateString();
    return (
        <div>
            <h3>Membership</h3>
            id: {props.membership.id}
            <br />
            Locations:
            <ul>
                {props.membership.location.map((loc) => (
                <li key={loc.toString()}>{loc}</li>
            ))}
            start: {start}
            <br />
            exp: {end}
            </ul>
        </div>
    )
}

export default function User(props) {
    return(
        <div className="user-container"> 

            <Name fname={props.user.fname} lname={props.user.lname}/>
            <Username username={props.user.username}/>
            <Contact contact={props.user.contact}/>
            <Membership membership={props.user.membership}/>

        </div>


    )
}

