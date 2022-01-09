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
            {/* {cont} */}
            {props.contact.email}
            <br />
            {props.contact.phone && props.contact.phone}
            
        </div>
    );

    
}


export default function User(props) {
    return(
        <div className="user-container"> 

            <Name fname={props.user.fname} lname={props.user.lname}/>
            <Username username={props.user.username}/>
            <Contact contact={props.user.contact}/>

        </div>


    )
}

