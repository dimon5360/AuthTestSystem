
function logout() {

    const xhr = new XMLHttpRequest();

    xhr.addEventListener("readystatechange", function () {
        if (this.status === 200) {
            console.log("got response");
            window.location.href=`/`;
            return this.responseText;
        }
    });

    xhr.open("POST", "/api/v1/auth");
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(JSON.stringify(document.write("hello: hello")));
}