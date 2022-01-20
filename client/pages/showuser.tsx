import React, { useState, useEffect } from 'react'
import axiod from "axiod";
import User from '~/components/user.tsx';
// import * as User from '~/components/user.tsx';
import "../style/userdata.css"

// TODO: figure out how to simplify datetime 
const url = "http://localhost:8080/api/dummy_user";
// const url = "http://localhost:8000/api/dummy_user";

export default function ShowUser() {

    const [userData, setUserData] = useState([]);
    const [done, setDone] = useState(undefined);
    

    const getUser  = async () => { 
        const response = await axiod.get(url);
        
        setUserData(response.data);
        setDone(true);
    };

    useEffect(() => {
        getUser();
    }, []);


        
    // TODO: Make  functions that will extract the optional data

    return !done?("Loading..."):(
        
        <div className="showuser">
            <header className="showuser-header">
                <h2>User Data</h2>
            </header>
        
        <User user={userData}/>
        {/* <User.Name fname={userData.fname} lname={userData.lname}/> */}
        </div>
    );

}