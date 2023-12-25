'use client';
import React, { useEffect, useState } from 'react';
import axios from 'axios';

interface Post {
  id: number;
  title: string;
  body: string;
}


export default function Home() {
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
      const fetchPosts = async () => {
          setLoading(true);
          try {
              const response = await axios.get('http://localhost:8080/posts');
              setPosts(response.data);
          } catch (error) {
              console.error('Error fetching posts:', error);
          } finally {
              setLoading(false);
          }
      };

      fetchPosts();
  }, []);

  if (loading) {
      return <p>Loading posts...</p>;
  }

  return (
      <div>
          {posts.map(post => (
              <div key={post.id}>
                  <h3>{post.title}</h3>
                  <p>{post.body}</p>
              </div>
          ))}
      </div>
  );
}
