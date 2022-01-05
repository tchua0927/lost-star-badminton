import React, { useState, useEffect } from 'react'
import axiod from "axiod";
import "../style/userdata.css"

// TODO: figure out how to simplify datetime 
const url = "http://localhost:8080/api/dummy_user";
// const url = "http://localhost:8000/api/dummy_user";

export default function ShowUser() {

    const [userData, setUserData] = useState([]);

    const getUser  = async () => { 
        const response = await axiod.get(url);
        setUserData(response.data)


    };

    useEffect(() => {
        getUser();
    }, []);

    if(userData.membership) {
        console.log(userData.membership);
    }
    
    

    return (
        <div className="showuser">
            <header className="showuser-header">
                <h2>User Data</h2>
            </header>

            <div className="user-container"> 

            {/* TODO: WORK ON CONTROL FLOW */}
                {/* <h5 className="info-item">{userData.fname}</h5> */}
                {/* <h5 className="info-item">{userData.lname}</h5> */}
                {userData.fname} 
                <br />
                {userData.lname}
                <br />
                {/* {userData.username && <h5 className="info-item">{userData.username}</h5>} */}
                {userData.username && userData.username}
                <br />
                {
                    userData.contact && 
                    (userData.contact.email &&
                    (
                        userData.contact.phone &&
                        userData.contact.phone
                    )
                    )
                    // Object.keys(userData.contact).map((key,i) => (
                    //     userData.contact[key] &&
                    //     // <h5 className="info-item" key={i}>{userData.contact[key]}</h5>
                    // ))
                }
                {
                    userData.membership &&
                    
                    <h5 className="info-item" >{
                        userData.membership.id
                        } <br />
                        {userData.membership.location[0]}</h5>
                    
                }
                {/* {
                    userData.membership && 
                    Object.keys(userData.membership).map((key,i) => (
                        userData.membership[key] &&
                        <h5 className="info-item" key={i}>{userData.membership[key]}</h5>
                        // <h5 className="info-item" key={i}>{i}</h5>

                    ))
                } */}
                {/* {userData.membership && <h5 className="info-item">{userData.membership}</h5>} */}

            </div>

        </div>
    );

}