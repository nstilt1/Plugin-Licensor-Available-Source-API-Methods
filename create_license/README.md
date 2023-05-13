# API Documentation
The store must send information with purchases.

struct MyRequest {
    company: String,
    products_info: String,
    order_number: String,
    first_name: String,
    last_name: String,
    email: String,
    timestamp: String,
    signature: String
}

# products_info
This will be a string that contains all information about the purchase:

[productID],[licenseType],[quantity],[subtotal];(continues for each product)

## Restrict customer from purchasing different types of licenses for the same plugin at the same time!!!

# signature
The signature will be signed to every body parameter concatenated in order

# email
The email must be hashed. If you use an email address there, it will return a 403 response.