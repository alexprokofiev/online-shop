const router = async route => {
    let content = document.getElementById("root");
    content.innerHTML = "";

    let url = route.startsWith("#") ? route.slice(1) : route;

    if (url === "") {
        content.innerHTML = await (await fetch("/home")).text();

        return;
    }

    if (url === "/") {
        url = "/home";
    }

    const response = await fetch(url);

    const body = await response.text();

    content.innerHTML = body;
};

const init = () => {
    router(window.location.hash);

    window.addEventListener("hashchange", () => {
        router(window.location.hash);
    });
};

window.addEventListener("load", init);


function addToCart(productId) {
    const data = new URLSearchParams();

    data.append("product_id", productId);
    fetch('/add_to_cart', {
        method: 'POST',
        body: data,
    });

    return false;
};

function q() {
    let res = 0;
    document.getElementsByName("itemPrice").forEach(
        el => {
            res += +el.innerText * +el.parentElement.nextSibling.nextSibling.nextSibling.nextSibling.value;

            const data = new URLSearchParams();

            data.append("product_id", +el.parentElement.nextSibling.nextSibling.nextSibling.nextSibling.dataset.productId);
            data.append("quantity", el.parentElement.nextSibling.nextSibling.nextSibling.nextSibling.value);

            fetch('/change_quantity', {
                method: 'POST',
                body: data,
            });
        }
    );

    document.getElementById("totalCostValue").innerText = res + " ₽";
}

function order() {
    fetch('/order', {
        method: 'POST',
        body: "",
    });

    window.location.href = "/";
}

const clearCart = () => {
    fetch('/clear_cart', {
        method: 'POST',
        body: "",
    });

    window.location.href = "/";
};