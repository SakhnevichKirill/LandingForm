/**
 * These annotations control how your component sizes
 * Learn more: https://www.framer.com/docs/guides/auto-sizing
 *
 * @framerSupportedLayoutWidth any-prefer-fixed
 * @framerSupportedLayoutHeight any-prefer-fixed
 */
export default function Custom_form(props) {
    // This is a React component containing an Example component
    // - Replace <Example /> with your own code
    // - Find inspiration: https://www.framer.com/developers/

    return (
        // <div style={containerStyle}>
        //     <Example />
        // </div>
        <form
            // here you have to paste your individual link from f.e. formspark
            action="https://submit-form.com/lNAsHO9O"
            style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "left",
                fontFamily: "Inter, sans-serif",
                fontSize: "18px",
                width: "100%",
                color: "white",
                // padding: "24px",
                // backgroundColor: "white",
                borderRadius: "8px",
                boxShadow: "0 0 8px 0 rgba(0, 0, 0, 0.0)",
            }}
        >
            <input
                type="hidden"
                name="_redirect"
                // here you have to paste link, where do you want to lead your users to
                value=""
            />
            <label htmlFor="name" style={{ marginBottom: "8px" }}>
                Имя:
            </label>
            <input
                type="text"
                id="name"
                name="name"
                placeholder="Имя"
                required
                style={{
                    color: "white",
                    backgroundColor: "#1A1A1A",
                    width: "100%",
                    padding: "16px",
                    marginBottom: "24px",
                    border: "1px solid #ccc",
                }}
            />
            <label htmlFor="email" style={{ marginBottom: "8px" }}>
                Email:
            </label>
            <input
                type="text"
                id="email"
                name="email"
                placeholder="Email"
                required
                style={{
                    color: "white",
                    backgroundColor: "#1A1A1A",
                    width: "100%",
                    padding: "16px",
                    marginBottom: "24px",
                    border: "1px solid #ccc",
                    borderRadius: "4px",
                }}
            />
            <label htmlFor="phonenumber" style={{ marginBottom: "8px" }}>
                Номер телефона:
            </label>
            <input
                type="text"
                id="number"
                name="number"
                placeholder="Номер"
                required
                style={{
                    color: "white",
                    backgroundColor: "#1A1A1A",
                    width: "100%",
                    padding: "16px",
                    marginBottom: "24px",
                    border: "1px solid #ccc",
                    borderRadius: "4px",
                }}
            />
            <input
                type="submit"
                value="Получить премиум"
                style={{
                    fontSize: "18px",
                    fontFamily: "Montserrat",
                    width: "100%",
                    padding: "16px",
                    backgroundColor: "#0C89E9",
                    color: "white",
                    fontWeight: "bold",
                    border: "none",
                    borderRadius: "4px",
                    cursor: "pointer",
                }}
            />
        </form>
    )
}