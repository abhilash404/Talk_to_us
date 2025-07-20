document.addEventListener("DOMContentLoaded", function () {
    // Get form elements
    const usernameInput = document.getElementById("username");
    const emailInput = document.getElementById("email");
    const passwordInput = document.getElementById("password");
    const registerButton = document.querySelector("button[type='submit']");

    // Create a message display if it does not exist
    let messageDiv = document.getElementById('register-message');
    if (!messageDiv) {
        messageDiv = document.createElement('div');
        messageDiv.id = 'register-message';
        messageDiv.style.marginTop = '1em';
        messageDiv.style.textAlign = 'center';
        document.getElementById("form-container").appendChild(messageDiv);
    }

    registerButton.addEventListener("click", async function (e) {
        e.preventDefault();

        // Grab and trim input values
        const username = usernameInput.value.trim();
        const email = emailInput.value.trim();
        const password = passwordInput.value;

        messageDiv.style.color = "#d00";
        messageDiv.textContent = "";

        // Basic validation
        if (!username || !email || !password) {
            messageDiv.textContent = "Please fill in all fields.";
            return;
        }

        registerButton.disabled = true;
        registerButton.textContent = "Registering...";

        try {
            // Send valid JSON body with all keys double-quoted
            const payload = {
                username: username,
                email: email,
                password: password,
                role: "customer"
            };

            const response = await fetch("http://127.0.0.1:3000/register", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(payload)
            });

            // Try to parse response as JSON, but fallback if not possible
            let data = "";
            try {
                data = await response.json();
            } catch {
                data = await response.text();
            }

            if (response.ok) {
                messageDiv.style.color = "#080";
                messageDiv.textContent = "Registration successful!";
                usernameInput.value = "";
                emailInput.value = "";
                passwordInput.value = "";

                // Optionally redirect after registration (give user brief success message before redirect)
                setTimeout(() => { window.location.href = "/frontend/index.html"; }, 1200);
            } else {
                messageDiv.style.color = "#d00";
                if (response.status === 409) {
                    // Email already exists
                    if (typeof data === "string") {
                        messageDiv.textContent = data;
                    } else {
                        messageDiv.textContent = data.message || "Email already registered.";
                    }
                } else if (data && data.message) {
                    messageDiv.textContent = data.message;
                } else if (typeof data === "string") {
                    messageDiv.textContent = data; // fallback to plain string error
                } else {
                    messageDiv.textContent = "Error during registration.";
                }
            }
        } catch (err) {
            messageDiv.style.color = "#d00";
            messageDiv.textContent = "Network or server error.";
        }

        registerButton.disabled = false;
        registerButton.textContent = "Register";
    });
});
