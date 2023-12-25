'use client';
import React, { useEffect, useState } from 'react';
import axios from 'axios';


export default function Home() {
  const [message, setMessage] = useState('');

useEffect(() => {
  const fetchMessage = async () => {
    try {
      const response = await axios.get('http://localhost:8080/hello/world');
      setMessage(response.data);
      console.log(response.data);
    } catch (error) {
      console.error('APIエラー:', error);
    }
  };

  fetchMessage();
}, []);
  return (
    <div>
      <p>{message}</p>
    </div>
  )
}
