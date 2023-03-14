package main

import (
	"fmt"
	"html/template"
	"log"
	"net/http"
)

type User struct {
	ID    int
	Name  string
	Email string
}

var users []User

func indexHandler(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title string
		Users []User
	}{
		Title: "List of Users",
		Users: users,
	}

	tmpl := template.Must(template.ParseFiles("layout.html", "index.html"))

	err := tmpl.ExecuteTemplate(w, "layout.html", data)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func newHandler(w http.ResponseWriter, r *http.Request) {
	data := struct {
		Title string
		User  User
	}{
		Title: "Add New User",
		User:  User{},
	}

	tmpl := template.Must(template.ParseFiles("layout.html", "form.html"))

	err := tmpl.ExecuteTemplate(w, "layout.html", data)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func createHandler(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	user := User{
		ID:    len(users) + 1,
		Name:  r.Form.Get("name"),
		Email: r.Form.Get("email"),
	}

	users = append(users, user)

	http.Redirect(w, r, "/", http.StatusSeeOther)
}

func editHandler(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Path[len("/edit/"):]

	var id int

	fmt.Sscanf(idStr, "%d", &id)

	var user User

	for _, u := range users {
		if u.ID == id {
			user = u
			break
		}
	}

	data := struct {
		Title string
		User  User
	}{
		Title: "Edit User",
		User:  user,
	}

	tmpl := template.Must(template.ParseFiles("layout.html", "form.html"))

	err := tmpl.ExecuteTemplate(w, "layout.html", data)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func updateHandler(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	idStr := r.URL.Path[len("/update/"):]

	var id int

	fmt.Sscanf(idStr, "%d", &id)

	fmt.Println(idStr)
	fmt.Println(r.Form.Get("name"))
	for i := 0; i < len(users); i++ {
		if users[i].ID == id {
			users[i].Name = r.Form.Get("name")
			users[i].Email = r.Form.Get("email")
			break
		}
	}

	http.Redirect(w, r, "/", http.StatusSeeOther)
}

func deleteHandler(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Path[len("/delete/"):]
	var id int
	fmt.Sscanf(idStr, "%d", &id)

	for i := 0; i < len(users); i++ {
		if users[i].ID == id {
			copy(users[i:], users[i+1:])
			users = users[:len(users)-1]
			break
		}
	}

	http.Redirect(w, r, "/", http.StatusSeeOther)
}

func main() {

	http.HandleFunc("/", indexHandler)
	http.HandleFunc("/new", newHandler)
	http.HandleFunc("/create", createHandler)
	http.HandleFunc("/edit/", editHandler)
	http.HandleFunc("/update/", updateHandler)
	http.HandleFunc("/delete/", deleteHandler)

	log.Fatal(http.ListenAndServe(":8080", nil))
}
