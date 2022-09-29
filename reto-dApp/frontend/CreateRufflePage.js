const CreateRufflePage = () => {
  return (
    <div className="create-ruffle-page">
      <h2>Create your decentralized Ruffle</h2>
      <form action="">
        <legend>Set up your Ruffle</legend>
        <input
          type="text"
          name="ruffle-title"
          id="ruffle-title"
          placeholder="Ruffle Title"
        />
        <button>Create Ruffle</button>
      </form>
    </div>
  );
};
export default CreateRufflePage;
