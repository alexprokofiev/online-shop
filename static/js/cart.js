document.addEventListener('DOMContentLoaded', function() {
    const increaseButtons = document.querySelectorAll('.increase');
    const decreaseButtons = document.querySelectorAll('.decrease');
    const removeButtons = document.querySelectorAll('.remove');
    const clearCartButton = document.getElementById('clear-cart');
    const summaryContainer = document.getElementById('summary');
    const totalItemsCountElement = document.getElementById('totalItemsCount');
    const totalOrderSumElement = document.getElementById('totalOrderSum');

    let totalItemsCount = 0;
    let totalOrderSum = 0;

    // Пример данных товаров
    const products = [
        { id: 1, name: 'Товар 1', price: 100, size: 'S' },
        // Добавьте больше товаров по необходимости
    ];

    // Функция для увеличения количества товара
    function increaseQuantity(button) {
        const quantityInput = button.nextElementSibling;
        const currentQuantity = parseInt(quantityInput.textContent);
        quantityInput.textContent = currentQuantity + 1;
        updateSummary();
    }

    // Функция для уменьшения количества товара
    function decreaseQuantity(button) {
        const quantityInput = button.previousElementSibling;
        const currentQuantity = parseInt(quantityInput.textContent);
        if (currentQuantity > 1) {
            quantityInput.textContent = currentQuantity - 1;
        }
        updateSummary();
    }

    // Функция для удаления товара
    function removeProduct(button) {
        const cartItem = button.closest('.cart-item');
        cartItem.remove();
        updateSummary();
    }

    // Функция для обновления общего количества товаров и общей суммы заказа
    function updateSummary() {
        const items = document.querySelectorAll('.cart-item');
        totalItemsCount = items.length;
        totalOrderSum = items.reduce((sum, item) => sum + parseFloat(item.querySelector('.price').textContent.replace('₽', '')), 0);
        totalItemsCountElement.textContent = totalItemsCount;
        totalOrderSumElement.textContent = totalOrderSum.toFixed(2) + ' ₽';
    }

    // Обработчики событий
    increaseButtons.forEach(increaseButton => {
        increaseButton.addEventListener('click', () => increaseQuantity(increaseButton));
    });

    decreaseButtons.forEach(decreaseButton => {
        decreaseButton.addEventListener('click', () => decreaseQuantity(decreaseButton));
    });

    removeButtons.forEach(removeButton => {
        removeButton.addEventListener('click', () => removeProduct(removeButton));
    });

    // Обработчик очистки корзины
    clearCartButton.addEventListener('click', function() {
        const cartItems = document.querySelectorAll('.cart-item');
        cartItems.forEach(cartItem => cartItem.remove());
        totalItemsCount = 0;
        totalOrderSum = 0;
        totalItemsCountElement.textContent = totalItemsCount;
        totalOrderSumElement.textContent = totalOrderSum.toFixed(2) + ' ₽';
    });

    // Обновляем резюме при загрузке страницы
    updateSummary();
});
