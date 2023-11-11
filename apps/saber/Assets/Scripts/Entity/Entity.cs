using Cinemachine;
using UnityEngine;
using UnityEngine.AI;

public class Entity : MonoBehaviour
{
  #region Entity
  public int energy;

  //? Camera
  protected Camera mainCamera;
  public CinemachineVirtualCamera virtualCamera;

  //? Name
  public string Name { get; set; } // Adding a Name property

  //? Health
  private int health;
  private EntityHealthBar healthBar;
  public int Health
  {
    get => health;
    set
    {
      health = Mathf.Max(0, value);
      UpdateHealthBar();
    }
  }

  //? Mana
  private int mana;
  public int Mana
  {
    get => mana;
    protected set => mana = Mathf.Max(0, Mathf.Min(value, MaxMana));
  }

  //? Movement
  public Vector3 position;
  private NavMeshAgent navMeshAgent;
  private float _moveSpeed = 5f;
  public float MoveSpeed
  {
    get => _moveSpeed;
    set
    {
      _moveSpeed = value;
      if (navMeshAgent != null)
      {
        navMeshAgent.speed = _moveSpeed; // Update the NavMeshAgent's speed
      }
    }
  }

  //? Stats
  public int MaxMana { get; protected set; }
  public int Strength { get; protected set; }
  public int Agility { get; protected set; }
  public int Intelligence { get; protected set; }
  public int Experience { get; protected set; }
  public int Reputation { get; protected set; }

  #endregion


  #region Core

  void Start()
  {
    InitializeEntity();
    InitializeCamera();
    InitializeNavMeshAgent();
    InitializeHealthBar();
  }

  void Update() { }
  #endregion

  #region Cycles



  #endregion

  #region Initialization

  private void InitializeCamera()
  {
    mainCamera = Camera.main;

    virtualCamera = FindObjectOfType<CinemachineVirtualCamera>();
  }

  protected virtual void InitializeEntity()
  {
    // Default values
    Name = "Entity"; // Default name
    Health = 100;
    MaxMana = 50;
    Mana = MaxMana;
    Strength = 10;
    Agility = 10;
    Intelligence = 10;
    position = transform.position;
    Experience = 0; // Default experience
    Reputation = 0; // Default reputation
  }

  private void InitializeHealthBar()
  {
    healthBar = gameObject.AddComponent<EntityHealthBar>(); // Add HealthBar component
    if (virtualCamera != null)
    {
      healthBar.InitializeHealthBar(virtualCamera);
    }
  }

  private void InitializeNavMeshAgent()
  {
    navMeshAgent = GetComponent<NavMeshAgent>();
    if (navMeshAgent == null)
    {
      navMeshAgent = gameObject.AddComponent<NavMeshAgent>();
    }

    navMeshAgent.speed = MoveSpeed;
    // Any additional NavMeshAgent configuration goes here
  }

  #endregion

  #region Movement

  public virtual void Move(Vector3 targetPosition)
  {
    if (navMeshAgent != null)
    {
      navMeshAgent.SetDestination(targetPosition);
    }
  }

  #endregion

  #region Combat

  public virtual void TakeDamage(int amount)
  {
    //TODO Debuffs / Enchants
    Health -= amount;
  }

  public virtual void Heal(int amount)
  {
    //TODO Heal Regen
    Health += amount;
  }

  public virtual void UseMana(int amount)
  {
    Mana -= amount;
  }

  public virtual void RestoreMana(int amount)
  {
    Mana += amount;
  }

  public void GainExperience(int amount)
  {
    Experience += amount;
  }

  public void ChangeReputation(int amount)
  {
    Reputation += amount;
  }

  private void UpdateHealthBar()
  {
    if (healthBar != null)
    {
      float healthNormalized = (float)health; // Health (Int) to Float
      healthBar.SetHealth(healthNormalized);
    }
  }

  #endregion
}