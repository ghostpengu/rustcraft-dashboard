function submit() {
    const type = document.getElementById("type");
    const email = document.getElementById("email");
    const version = document.getElementById("version");
    let typestr = type.value;
    let versionstr = version.value;
    const data = {
        version: versionstr,
        stype:typestr,
        email:email.value,
        setup:"false",
    };
    console.log(data);
    fetch('/user/setproperties/'+token, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      })
        .then(response => response.json())

  fetchsetup();
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

  
const token = getCookie("token");
function fetchsetup(){
  fetch("/user/setup/" + token)
  .then((response) => {
    // Check if the request was successful (status code 200)
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }
    return response.text(); // Retrieve the raw text content
  })
  .then((data) => {
    // This function will be executed when the request is successful
    console.log("Data:", data);

    // Use the data as needed, for example, updating the DOM
    if(data === "false"){
      window.location.href = "/";
    }
   
  })
  .catch((error) => {
    // This function will be executed if there is an error
    console.error("Error:", error);
  });
 
}
document.addEventListener("DOMContentLoaded", function () {
 
  fetchsetup();
    
  });
  