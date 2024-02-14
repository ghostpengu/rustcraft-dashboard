document.addEventListener("DOMContentLoaded", function() {
  let send = document.getElementById("submit");
  send.addEventListener("click", function () {
    // This function will be executed when the button is clicked
    function setCookie(name, value, days) {
      const date = new Date();
      date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
      const expires = "expires=" + date.toUTCString();
      document.cookie = name + "=" + value + ";" + expires + ";path=/";
    }
  
    // Example usage

    const expirationDays = 360; // Expires in 30 days
  
    // Set the cookie with the name "my_cookie" and value "Hello, World!"
  
    fetch(
      "/createuser/" +
      document.getElementById("username").value +
      "/" +
      document.getElementById("password").value
    )
      .then((response) => {
        // Check if the request was successful (status code 200)
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        return response.json(); // Retrieve the raw text content
      })
      .then((data) => {
        // This function will be executed when the request is successful
        const username = data.username;
        const token = data.token;
        // Store values in letiables or use them as needed
        console.log('Value of username:', username);
        console.log('Do not share :', token);
  
        // Use the data as needed, for example, updating the DOM
        setCookie("username", username, expirationDays);
        setCookie("token", token, expirationDays);
      })
      .catch((error) => {
        // This function will be executed if there is an error
        console.error("Error:", error);
      });
    window.location.href = "/";
  });
  
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
  
  // Example usage
  const cookieValue = getCookie("token"); // get the value of the cookie named "my_cookie"
  console.log(cookieValue); // print the value to the console
  
});
