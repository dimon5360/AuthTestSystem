
function auth() {

    const xhr = new XMLHttpRequest();

    const json = {
        login: document.getElementById('Login').value,
        password: document.getElementById('Password').value,
    };

    xhr.addEventListener("readystatechange", function () {
        if (this.readyState === 4) {
            console.log("got response");
            window.location.href=`/main/${json.login}`;
            return this.responseText;
        }
    });

    xhr.open("POST", "/api/v1/auth/login");
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(JSON.stringify(json));
}