window.onload = () => {
    const openSearch = () => {
        document.getElementById("search-input").focus();
    };

    document.getElementById("search").addEventListener(
        "click",
        openSearch
    );
};