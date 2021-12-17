import React, { useState, useEffect } from 'react'
import axiod from "axiod";
import "../style/userdata.css"

const url = "http://localhost:8080/api/dummy_user";

export default function ShowUser() {

    const [userData, setUserData] = useState([]);

    const getGiHubUserWithAxios  = async () => { 
        const response = await axiod.get(url);
        setUserData(response.data);
    };

    useEffect(() => {
        getGiHubUserWithAxios()
    }, []);
    

    return (
        <div className="showuser">
            <header className="showuser-header">
                <h2>User Data</h2>
            </header>

            <div className="user-container"> 
                <h5 className="info-item">{userData.username}</h5>
                <h5 className="info-item">{userData.fname}</h5>
                <h5 className="info-item">{userData.lname}</h5>
                {/* {userData.map(contact => <div>{contact}</div>)} */}
                {/* <h5 className="info-item">{JSON.stringify(userData.contact)}</h5> */}
            </div>

        </div>
    );

}