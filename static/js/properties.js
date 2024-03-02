function setproperties() {
    const editor = document.querySelector(".editor");
    const data = {
      out: editor.value
    };
  
    if (editor) {
      let serverprop = "";
     
      fetch('/setproperties/'+token, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      })
        .then(response => response.json())
       
    
    }
  }
  function getCookie(name) {
    const cookieName = name + "="; // the name of the cookie followed by "="
    const decodedCookie = decodeURIComponent(document.cookie); // decode the cookie string
    const cookieArray = decodedCookie.split(";"); // split the string into an array of cookies
    for (let i = 0; i < cookieArray.length; i++) {
      // loop through each cookie
      let cookie = cookieArray[i].trim(); // remove leading and trailing whitespaces
      if (cookie.indexOf(cookieName) === 0) {
        // if this is the cookie we're looking for
        return cookie.substring(cookieName.length, cookie.length); // return its value
      }
    }
    return ""; // return empty string if cookie not found
  }
  function loadproperties() {
    const editor = document.querySelector(".editor");
  
    if (editor) {
      let serverprop = "";
      
      fetch("/getproperties/" + token)
        .then((response) => {
          if (!response.ok) {
            throw new Error("Network response was not ok");
          }
          return response.text();
        })
        .then((data) => {
         editor.value = data;
        
    
        })
      
  
      
    }
  
  }

  const token = getCookie("token");
document.addEventListener("DOMContentLoaded", function () {

  loadproperties();
  let usernametext = document.getElementById("username");
  usernametext.textContent = getCookie("username");

  
});
