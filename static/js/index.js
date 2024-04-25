
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
document.addEventListener("DOMContentLoaded", function () {


  setInterval(function() {
    // Your code here
    fetch("/read/" + token)
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
      document.getElementById("output").textContent = data;
    })
    .catch((error) => {
      // This function will be executed if there is an error
      console.error("Error:", error);
    });
  }, 5000); // 5000 milliseconds = 5 seconds
  let usernametext = document.getElementById("username");
  usernametext.textContent = getCookie("username");
  fetch("/checklogin/" + token)
  .then((response) => {
    // Check if the request was successful (status code 200)
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }
    return response.text(); // Retrieve the raw text content
  })
  .then((data) => {
    // This function will be executed when the request is successful


    // Use the data as needed, for example, updating the DOM
    if(data === "false"){
      window.location.href = "/login";
    }
   
  })
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


    // Use the data as needed, for example, updating the DOM
    if(data === "true"){
      window.location.href = "/setup";
    }
   
  })
  .catch((error) => {
    // This function will be executed if there is an error
    console.error("Error:", error);
  });

  fetch("/read/" + token)
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
      document.getElementById("output").textContent = data;
    })
    .catch((error) => {
      // This function will be executed if there is an error
      console.error("Error:", error);
    });
  let send = document.getElementById("sendc");
  send.addEventListener("click", function () {
    // This function will be executed when the button is clicked
    if (document.getElementById("command").value === "exit") {
      fetch("/exit/" + token);
    } else {
      fetch("/c/" + document.getElementById("command").value + "/" + token);
      setTimeout(function () {
        fetch("/read/" + token)
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
            document.getElementById("output").textContent = data;
          })
          .catch((error) => {
            // This function will be executed if there is an error
            console.error("Error:", error);
          });
      }, 30);

    }
  });
  let init = document.getElementById("init");
  init.addEventListener("click", function () {
    // This function will be executed when the button is clicked
    fetch("init/" + token);
  });
  let button = document.getElementById("read");

  // Add a click event listener to the button
  button.addEventListener("click", function () {
    // This function will be executed when the button is clicked
    fetch("/read/" + token)
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
        document.getElementById("output").textContent = data;
      })
      .catch((error) => {
        // This function will be executed if there is an error
        console.error("Error:", error);
      });
  });

  let start = document.getElementById("start");
  start.addEventListener("click", function () {
    // This function will be executed when the button is clicked
    fetch("/start/" + token);
  });

  let exit = document.getElementById("exit");
  exit.addEventListener("click", function () {
    // This function will be executed when the button is clicked
    fetch("/exit/" + token);
  });
});
