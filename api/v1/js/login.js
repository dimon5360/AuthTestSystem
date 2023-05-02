
function login() {

    const xhr = new XMLHttpRequest();

    const json = {
        login: document.getElementById('Login').value,
        password: document.getElementById('Password').value,
    };

    xhr.addEventListener("readystatechange", function () {
        if (this.status === 200) {
            console.log("got response");
            window.location.href=`/api/v1/main/${json.login}`;
            return this.responseText;
        }
    });

    xhr.open("POST", "/api/v1/login");
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(JSON.stringify(json));
}